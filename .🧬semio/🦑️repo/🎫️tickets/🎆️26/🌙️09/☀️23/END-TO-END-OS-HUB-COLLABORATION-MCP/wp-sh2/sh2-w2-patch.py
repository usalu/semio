#!/usr/bin/env python3
"""🗑️ SH2 W2 patch (hub): deleting a space first revokes every membership (`member.removed` per member, then `space.deleted`),
so each member's global directory socket owes its reader the existing `access-changed: revoked` frame and the reader's Home
re-reads its directory from the origin — a deleted space leaves the live Home at once instead of staying listed until a reload
(the page and socket filters are member-only, so `space.deleted` alone reached nobody). Adds the decider law and the live
socket law. Dry run by default; `--write` applies; `--root <dir>` targets a scratch overlay instead of the repo tree."""
import argparse, os, sys

REPO = "/Users/ueli/Documents/semio"
DECIDE = "🌎️hub/📇️directory/🦀️.rs"
DECIDE_TESTS = "🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs"
BIN_TESTS = "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"

EDITS = [
    (DECIDE, """/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
""", """/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
/// - `delete-space` removes every membership (owner included) before the space: raw directory events are member-only, so
///   `space.deleted` alone reaches no reader, while each `member.removed` owes its member's global socket an
///   `access-changed: revoked` frame and that reader re-reads its directory from the origin without the space.
"""),
    (DECIDE, """        DirectoryCommand::DeleteSpace { space_id } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceDeleted { space_id }))
        }
""", """        DirectoryCommand::DeleteSpace { space_id } => {
            require_space(dir, &space_id).await?;
            let mut events: Vec<NewDirectoryEvent> = dir.list_members(&space_id).await?.into_iter().map(|(user, _)| new_event(clock, actor, Some(space_id.clone()), Some(user.id.clone()), DirectoryEventBody::MemberRemoved { space_id: space_id.clone(), user_id: user.id })).collect();
            events.push(new_event(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceDeleted { space_id }));
            Ok(Decision { events, result: None })
        }
"""),
    (DECIDE_TESTS, """// 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
""", """// 🔬️ Decider law: `delete-space` revokes every membership (owner included) before it deletes the space, all under one
// decision — the member-only readers learn the deletion through their own `member.removed`.
#[tokio::test]
async fn delete_space_revokes_every_membership_before_the_space() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    let (events, _) = service.execute(owner, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
    let (last, removals) = events.split_last().expect("delete-space events");
    assert!(matches!(&last.body, DirectoryEventBody::SpaceDeleted { space_id: deleted } if *deleted == space_id), "the space is deleted last");
    let mut removed: Vec<&str> = removals.iter().map(|event| match &event.body {
        DirectoryEventBody::MemberRemoved { space_id: from, user_id } if *from == space_id && event.user_id.as_deref() == Some(user_id.as_str()) => user_id.as_str(),
        other => panic!("only member removals precede the deletion: {other:?}"),
    }).collect();
    removed.sort_unstable();
    assert_eq!(removed.len(), 2, "the owner and the added member are both removed: {removed:?}");
    assert!(removed.contains(&"u-owner"));
}

// 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
"""),
    (BIN_TESTS, """#[test]
fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
""", """/// 🗑️ Live on a global directory socket: a member of a space its owner deletes is told its access was revoked (its own
/// `member.removed` of the deletion), so its Home re-reads the directory from the origin without the space.
#[test]
fn a_member_of_a_deleted_space_is_told_its_access_was_revoked() {
    run_socket_test(|| async {
        let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("deleted-space state open deadline");
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let owner = issue_test_session(&state, "deleted-owner@example.test").await;
        let reader = issue_test_session(&state, "deleted-reader@example.test").await;
        let doomed = create_space_for_test(&state, &owner.user_id, "Doomed", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &doomed, "deleted-reader@example.test", DirectorySpaceRole::Spectator).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&reader.token), State(state.clone()), Bytes::new()).await.expect("reader grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut socket, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await.expect("reader socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("reader hello");
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#test", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: doomed.clone() }).await.expect("delete space");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: doomed.clone(), change: os_directory::DirectoryAccessChange::Revoked }, "the deletion revokes the member's access");
        socket.close(None).await.expect("close reader socket");
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
"""),
]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--root", default=REPO)
    arguments = parser.parse_args()
    texts: dict[str, str] = {}
    states = []
    for path, old, new in EDITS:
        full = os.path.join(arguments.root, path)
        text = texts.setdefault(path, open(full, encoding="utf-8").read())
        if new in text:
            states.append(("applied", path))
            continue
        if text.count(old) != 1:
            states.append((f"conflict: anchor found {text.count(old)}x", path))
            continue
        texts[path] = text.replace(old, new)
        states.append(("apply", path))
    for state, path in states:
        print(f"{state:>30}  {path}")
    if any(state.startswith("conflict") for state, _ in states):
        return 1
    if arguments.write:
        for path, text in texts.items():
            with open(os.path.join(arguments.root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
        print("written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
