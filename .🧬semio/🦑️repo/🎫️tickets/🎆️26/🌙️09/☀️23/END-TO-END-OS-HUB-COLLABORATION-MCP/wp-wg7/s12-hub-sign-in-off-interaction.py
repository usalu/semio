#!/usr/bin/env python3
"""🔐️ WG7 session 12: the hub sign-in and the spaces read never hold the frame's interaction state.

Measured (runs s12g/s12h, hub 7800 under load): `run_hub_sign_in_turn` awaited POST /auth/sessions, GET me and
GET /directory/spaces inline inside a frame-deferred action, which owns the interaction state for its whole run, so the
browser shell took NO input for 60-106 s (sign-in 11-30 s, spaces 33.6 s for 85 spaces). React stays interactive.

After this patch the network legs run on a spawned task (`ShellHubTask`: the shared pool's I/O lane natively, the page's
microtask queue in the browser); the action only arms it and returns, the chrome shows the phase that already exists
(`Signing in…` / `Anmeldung läuft…` with Cancel, spaces `Loading`), Cancel ends the task through its cancel token, and
the directory pump applies the answer on both targets. The spaces read moves to its own task, so a verified session is
shown the moment `me` answers.

Renderer-only (not guest-linked). Apply after the `wasmshort` lane reopens; dry run by default, `--apply` writes.
Every replacement asserts that its anchor occurs exactly once.
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SHELL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"

TASK_TYPES = '''const HUB_SIGN_IN_DEADLINE_MS: u64 = 30_000;

/// 🔐️ What one spawned sign-in answers: the mint's refusal, a mint the hub's own `me` read would not confirm, or the
/// verified session.
enum ShellHubSignInAnswer {
    Failed { code: HubSignInErrorCode, retry_after_seconds: Option<u64> },
    Unverified,
    Verified { origin: String, user_id: String, credential: std::sync::Arc<LocalHubCredential>, client: std::sync::Arc<ShellDirectoryClient>, authority: DirectorySessionAuthorityV1 },
}

/// 🏘️ What one spawned spaces read answers: the rows, or `Err` when the list could not be read (the rows on screen stay).
type ShellHubSpacesAnswer = Result<Vec<crate::space_browser::SpaceRow>, ()>;

/// 🔐️ One hub workspace request whose network legs run on a spawned task and never on the frame's interaction state:
/// the frame keeps taking input while the hub answers, the chrome shows the phase, and `cancel` ends the request. A
/// slow hub froze every input of the browser shell for 60–106 s while these legs were awaited inside the action
/// (ticket 26/09/23 session 12, runs s12g/s12h on hub 7800).
struct ShellHubTask<T> {
    receiver: std::sync::mpsc::Receiver<T>,
    cancel: CancelToken,
    #[cfg(not(target_arch = "wasm32"))]
    task: Option<std::sync::Arc<ShellPoolFuture>>,
}

impl<T: 'static> ShellHubTask<T> {
    /// 🚀️ Runs `leg` on the shared pool's I/O lane; the pump reads its answer.
    #[cfg(not(target_arch = "wasm32"))]
    fn spawn(cancel: CancelToken, leg: impl std::future::Future<Output = T> + Send + 'static) -> Self
    where
        T: Send,
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
            let _ = sender.send(leg.await);
        });
        Self { receiver, cancel, task: Some(task) }
    }

    /// 🚀️ Runs `leg` on the page's own microtask queue; the pump reads its answer.
    #[cfg(target_arch = "wasm32")]
    fn spawn(cancel: CancelToken, leg: impl std::future::Future<Output = T> + 'static) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        crate::spawn_app_task(async move {
            let _ = sender.send(leg.await);
        });
        Self { receiver, cancel }
    }

    /// 📬️ `Some(Ok)` once answered, `Some(Err)` when the task ended without an answer, `None` while it runs.
    fn answer(&self) -> Option<Result<T, ()>> {
        match self.receiver.try_recv() {
            Ok(answer) => Some(Ok(answer)),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(())),
        }
    }

    /// 🛑️ Ends the request: its transport sees the cancellation, and no answer is read any more.
    fn cancel(self) {
        self.cancel.cancel_now();
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(task) = self.task {
            task.cancel();
        }
    }
}'''

FIELDS = '''    pub hub_workspace: crate::hub_connection::HubWorkspaceState,
    /// 🔐️ The sign-in whose network legs are in flight off the interaction state ([`ShellHubTask`]).
    hub_sign_in_task: Option<ShellHubTask<ShellHubSignInAnswer>>,
    /// 🏘️ The spaces read in flight, on the same kind of task.
    hub_spaces_task: Option<ShellHubTask<ShellHubSpacesAnswer>>,'''

SIGN_IN_BODY = '''    /// 🔐️ Arms one credential sign-in against the SELECTED hub's own origin: the mint, then the `me` read that carries
    /// the deadline the mint answer deliberately omits (AU1 §1.1, AU3 §4.2), run as a [`ShellHubTask`] so the frame keeps
    /// taking input while the hub hashes the password. A second submit while one runs is ignored (the form disables it).
    ///
    /// 🔑️ The password draft is cleared the moment the attempt takes it: the one request that needs it owns it.
    fn start_hub_sign_in(&mut self) {
        if self.hub_sign_in_task.is_some() {
            return;
        }
        let origin = self.hub_workspace.origin().to_string();
        let credential = HubSignInCredential {
            email: self.hub_workspace.email_draft.clone(),
            password: std::mem::take(&mut self.hub_workspace.password_draft),
            device_instance_id: shell_hub_device_instance_id(&self.shell_session_id),
            client_class: if cfg!(target_arch = "wasm32") { HubSignInClientClass::Browser } else { HubSignInClientClass::Native },
        };
        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Submit);
        let mut ctx = self.directory_ctx();
        ctx.deadline_ms = Some(Self::directory_now_ms().saturating_add(HUB_SIGN_IN_DEADLINE_MS));
        let cancel = ctx.cancel.clone();
        let transport = self.directory_transport.clone();
        self.hub_sign_in_task = Some(ShellHubTask::spawn(cancel, async move {
            let result = match crate::hub_connection::run_hub_sign_in(&transport, &ctx, &origin, &credential).await {
                crate::hub_connection::HubSignInOutcome::Failed { code, retry_after_seconds } => return ShellHubSignInAnswer::Failed { code, retry_after_seconds },
                crate::hub_connection::HubSignInOutcome::Minted(result) => result,
            };
            let Ok(credential) = LocalHubCredential::from_minted_session(&origin, &result.token) else {
                return ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None };
            };
            let credential = std::sync::Arc::new(credential);
            let client = std::sync::Arc::new(DirectoryClient::authenticated(transport, credential.clone()));
            match client.me(&ctx).await {
                Ok(authority) if authority.user_id == result.user_id => ShellHubSignInAnswer::Verified { origin, user_id: result.user_id, credential, client, authority },
                Ok(_) | Err(_) => ShellHubSignInAnswer::Unverified,
            }
        }));
    }

    /// 🔐️ Applies one sign-in answer: a verified session owns the directory client, the socket grants and the book's
    /// last user, then the spaces read starts on its own task; a session verified for a hub no longer selected is
    /// dropped; anything else is the session's localized error.
    fn apply_hub_sign_in(&mut self, answer: ShellHubSignInAnswer) {
        match answer {
            ShellHubSignInAnswer::Failed { code, retry_after_seconds } => {
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code, retry_after_seconds });
            }
            ShellHubSignInAnswer::Verified { origin, user_id, credential, client, authority } if origin == self.hub_workspace.origin() => {
                self.identity = Some(Identity { user_id: authority.user_id.clone(), email: authority.email.clone(), display_name: authority.display_name.clone(), hub_base_url: origin, issued_at_ms: chrome_now_ms() as i64 });
                self.verified_session_authority = Some(authority);
                self.directory_client = Some(client.clone());
                self.document_host.set_local_hub_credential(credential);
                self.document_host.set_hub_socket_grant_source(client);
                let selected_id = self.hub_workspace.book.selected_id.clone();
                if let Some(connection) = self.hub_workspace.book.connections.iter_mut().find(|connection| connection.id == selected_id) {
                    connection.last_user_id = Some(user_id);
                }
                self.persist_hub_connection_book();
                self.project_verified_hub_authority();
                self.start_hub_spaces_reload();
            }
            ShellHubSignInAnswer::Verified { .. } => {}
            ShellHubSignInAnswer::Unverified => {
                self.clear_hub_session_owner();
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None });
            }
        }
    }

'''

SPACES_BODY = '''    /// 🏘️ Arms one spaces read on a [`ShellHubTask`] (a replaced read is cancelled); the list shows `Loading` and keeps
    /// the rows it had, which is the local-first rule the `Stale` phase already defends.
    fn start_hub_spaces_reload(&mut self) {
        if let Some(task) = self.hub_spaces_task.take() {
            task.cancel();
        }
        let Some(client) = self.directory_client.clone() else {
            self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Stale;
            return;
        };
        self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Loading;
        let ctx = self.directory_ctx();
        let cancel = ctx.cancel.clone();
        self.hub_spaces_task = Some(ShellHubTask::spawn(cancel, async move { client.spaces(&ctx).await.map(|entries| crate::space_browser::space_rows(&entries)).map_err(|_| ()) }));
    }

    /// 📬️ Reads the answers of the hub workspace's spawned requests, once each; answers whether the workspace changed.
    fn poll_hub_workspace_tasks(&mut self) -> bool {
        let mut changed = false;
        if let Some(answer) = self.hub_sign_in_task.as_ref().and_then(ShellHubTask::answer) {
            self.hub_sign_in_task = None;
            self.apply_hub_sign_in(answer.unwrap_or(ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::Unreachable, retry_after_seconds: None }));
            changed = true;
        }
        if let Some(answer) = self.hub_spaces_task.as_ref().and_then(ShellHubTask::answer) {
            self.hub_spaces_task = None;
            match answer.and_then(|rows| rows) {
                Ok(rows) => {
                    self.hub_workspace.rows = rows;
                    self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Ready;
                }
                Err(()) => self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Stale,
            }
            changed = true;
        }
        changed
    }

    /// 🔐️ Whether no hub workspace request is in flight.
    pub(crate) fn hub_workspace_settled(&self) -> bool {
        self.hub_sign_in_task.is_none() && self.hub_spaces_task.is_none()
    }

'''

SHELL_EDITS = [
    ("const HUB_SIGN_IN_DEADLINE_MS: u64 = 30_000;", TASK_TYPES),
    ("    pub hub_workspace: crate::hub_connection::HubWorkspaceState,", FIELDS),
    (
        "            hub_workspace: crate::hub_connection::HubWorkspaceState::new(crate::hub_sign_in::parse_hub_connection_book(prefs_get(HUB_CONNECTION_BOOK_STORAGE_KEY_V1).as_deref(), &shell_hub_bootstrap_origin(), ui_wgpu::wgpu::Locale::En)),",
        "            hub_workspace: crate::hub_connection::HubWorkspaceState::new(crate::hub_sign_in::parse_hub_connection_book(prefs_get(HUB_CONNECTION_BOOK_STORAGE_KEY_V1).as_deref(), &shell_hub_bootstrap_origin(), ui_wgpu::wgpu::Locale::En)),\n            hub_sign_in_task: None,\n            hub_spaces_task: None,",
    ),
    (
        """            hub_action::CANCEL_SIGN_IN => {
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::Cancelled, retry_after_seconds: None });
            }
            hub_action::SIGN_IN => self.run_hub_sign_in_turn().await,""",
        """            hub_action::CANCEL_SIGN_IN => {
                if let Some(task) = self.hub_sign_in_task.take() {
                    task.cancel();
                }
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::Cancelled, retry_after_seconds: None });
            }
            hub_action::SIGN_IN => self.start_hub_sign_in(),""",
    ),
    ("            hub_action::REFRESH_SPACES => self.reload_hub_spaces().await,", "            hub_action::REFRESH_SPACES => self.start_hub_spaces_reload(),"),
    (
        """    fn clear_hub_session_owner(&mut self) {
        self.directory_client = None;""",
        """    fn clear_hub_session_owner(&mut self) {
        if let Some(task) = self.hub_sign_in_task.take() {
            task.cancel();
        }
        if let Some(task) = self.hub_spaces_task.take() {
            task.cancel();
        }
        self.directory_client = None;""",
    ),
    (
        """        let creation_changed = self.pump_hub_artifact_creation().await;
        // 🌉️ Packet W15e: the agent bridge's socket rides the SAME 100 ms slot rather than a timer of
        // its own — one pump cadence for every out-of-process conversation this shell holds.
        let bridge_changed = self.pump_agent_bridge();
        identity_changed || administration_changed || creation_changed || bridge_changed""",
        """        let creation_changed = self.pump_hub_artifact_creation().await;
        let hub_changed = self.poll_hub_workspace_tasks();
        // 🌉️ Packet W15e: the agent bridge's socket rides the SAME 100 ms slot rather than a timer of
        // its own — one pump cadence for every out-of-process conversation this shell holds.
        let bridge_changed = self.pump_agent_bridge();
        identity_changed || administration_changed || creation_changed || hub_changed || bridge_changed""",
    ),
    (
        """        changed |= self.pump_hub_check_in().await;""",
        """        changed |= self.pump_hub_check_in().await;
        changed |= self.poll_hub_workspace_tasks();""",
    ),
]

LAW_EDITS = [
    (
        """    #[cfg(not(target_arch = "wasm32"))]
    drive(shell.handle_hub_workspace_action(verb, args));
    #[cfg(target_arch = "wasm32")]
    semio_framework_async::block_on(shell.handle_hub_workspace_action(verb, args));
}""",
        """    #[cfg(not(target_arch = "wasm32"))]
    drive(shell.handle_hub_workspace_action(verb, args));
    #[cfg(target_arch = "wasm32")]
    semio_framework_async::block_on(shell.handle_hub_workspace_action(verb, args));
    settle_hub_workspace(shell);
}

/// 🔐️ Pumps the directory lane until the hub workspace's spawned requests (sign-in, spaces) have answered — the
/// verbs only arm them now, and a frame applies the answer.
fn settle_hub_workspace(shell: &mut ShellState) {
    while !shell.hub_workspace_settled() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = crate::pump_renderer_io_sessions(1);
            drive(shell.pump_directory_events());
        }
        #[cfg(target_arch = "wasm32")]
        semio_framework_async::block_on(shell.pump_directory_events());
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}""",
    ),
]

LAW_APPEND = '''

/// 🔐️ LAW (ticket 26/09/23 session 12, runs s12g/s12h): the sign-in and the spaces read never hold the frame's
/// interaction state. The verbs only ARM a [`ShellHubTask`] and return — so the chrome shows `Signing in…` with its
/// Cancel while the hub hashes — the directory pump applies the answer, and Cancel ends the request: a cancelled
/// sign-in's late answer is never applied.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sign_in_verb_arms_a_task_and_returns_while_the_hub_is_still_answering() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let dispatch = source.split("async fn handle_hub_workspace_action(").nth(1).expect("the hub verb dispatch exists");
    let dispatch = &dispatch[..dispatch.find("\\n    }\\n").expect("the dispatch closes")];
    assert!(dispatch.contains("hub_action::SIGN_IN => self.start_hub_sign_in(),"), "SIGN_IN arms and returns");
    assert!(dispatch.contains("hub_action::REFRESH_SPACES => self.start_hub_spaces_reload(),"), "REFRESH_SPACES arms and returns");
    assert!(!source.contains("run_hub_sign_in_turn") && !source.contains("reload_hub_spaces().await"), "no hub leg is awaited inside a verb");

    let mut shell = shell();
    hub_verb(&mut shell, crate::hub_connection::action::SET_EMAIL, &[("value", "user1@semio.dev")]);
    hub_verb(&mut shell, crate::hub_connection::action::SET_PASSWORD, &[("value", "correct horse battery staple")]);
    let (sender, receiver) = std::sync::mpsc::channel();
    shell.hub_workspace.session = reduce_hub_session(&shell.hub_workspace.session, &HubSessionEvent::Submit);
    shell.hub_sign_in_task = Some(ShellHubTask { receiver, cancel: CancelToken::root_now(), task: None });
    assert_eq!(shell.hub_workspace.session.phase, HubSessionPhase::SigningIn);
    for locale in [Locale::En, Locale::De] {
        let tree = serde_json::to_string(&crate::hub_connection::build_hub_workspace_ui(&shell.hub_workspace, locale)).expect("hub tree json");
        assert!(tree.contains("framework.hub.sign-in.cancel"), "a running sign-in offers Cancel ({locale:?})");
    }
    assert!(!shell.poll_hub_workspace_tasks(), "nothing is applied before the hub answers");
    sender.send(ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::InvalidCredentials, retry_after_seconds: None }).expect("the task answers");
    assert!(shell.poll_hub_workspace_tasks(), "the pump applies the answer");
    assert!(shell.hub_workspace_settled());
    assert_eq!(shell.hub_workspace.session.error, Some(HubSignInErrorCode::InvalidCredentials));

    let (late, receiver) = std::sync::mpsc::channel();
    shell.hub_workspace.session = reduce_hub_session(&shell.hub_workspace.session, &HubSessionEvent::Submit);
    shell.hub_sign_in_task = Some(ShellHubTask { receiver, cancel: CancelToken::root_now(), task: None });
    hub_verb(&mut shell, crate::hub_connection::action::CANCEL_SIGN_IN, &[]);
    assert!(shell.hub_workspace_settled(), "Cancel ends the request");
    let _ = late.send(ShellHubSignInAnswer::Unverified);
    assert!(!shell.poll_hub_workspace_tasks(), "a cancelled sign-in's late answer is never applied");
    assert_eq!(shell.hub_workspace.session.error, Some(HubSignInErrorCode::Cancelled));
}
'''


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def cut_between(path, source, start_marker, end_marker, replacement):
    start = source.find(start_marker)
    end = source.find(end_marker, start)
    if start < 0 or end < 0 or source.count(start_marker) != 1:
        sys.exit(f"region {start_marker[:60]!r} .. {end_marker[:40]!r} not unique in {path.name}")
    return source[:start] + replacement + source[end:]


def main():
    apply = "--apply" in sys.argv
    shell_before = SHELL.read_text(encoding="utf-8")
    shell_after = replaced(SHELL, shell_before, SHELL_EDITS)
    shell_after = cut_between(SHELL, shell_after, "    /// 🔐️ One credential sign-in against the SELECTED hub's own origin", "    /// 🚪️ Sign-out.", SIGN_IN_BODY)
    shell_after = cut_between(SHELL, shell_after, "    async fn reload_hub_spaces(&mut self) {", "    /// 👥️ The open space's roster", SPACES_BODY)
    remaining = shell_after.count("self.reload_hub_spaces().await;")
    shell_after = shell_after.replace("self.reload_hub_spaces().await;", "self.start_hub_spaces_reload();")
    if "reload_hub_spaces()" in shell_after.replace("start_hub_spaces_reload()", "") or "run_hub_sign_in_turn" in shell_after:
        sys.exit("a caller of the removed inline legs is left")
    laws_before = LAWS.read_text(encoding="utf-8")
    laws_after = replaced(LAWS, laws_before, LAW_EDITS).rstrip("\n") + "\n" + LAW_APPEND
    for path, before, after in ((SHELL, shell_before, shell_after), (LAWS, laws_before, laws_after)):
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), path.name, path.name + " (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: 2 files, {remaining} further spaces callers now arm the task")


if __name__ == "__main__":
    main()
