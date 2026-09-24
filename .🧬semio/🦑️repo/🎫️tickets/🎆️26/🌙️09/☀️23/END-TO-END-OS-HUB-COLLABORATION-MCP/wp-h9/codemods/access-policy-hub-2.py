"""🛡️ One-off, pass 2: the document socket gate, Check In, artifact creation and directory commands."""
p = "/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)

rep('''    // 🔒️ Per-connection `SecurityGate`: `space_grants` compiles this space's `kind` into
    // author=rw/spectator=ro grants (archive additionally deny-overrides author writes), a fresh
    // `RoleBasedPolicy` from them, and a `Principal` carrying the caller's resolved role. A share-
    // token caller (no session role) is admitted as `"spectator"` — read-only, the
    // least-privilege default for a connection this crate cannot attribute to a real member.
    // `TenantId` reuses the space id: this crate has no separate tenant concept yet, and every
    // scope this gate ever evaluates already belongs to exactly this one space/document connection.
    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map_or_else(|| "studio".to_string(), |space| space.kind);
    let policy = db::security::space_grants(&space_id, &space_kind).await.into_iter().fold(db::security::RoleBasedPolicy::new(), db::security::RoleBasedPolicy::with_grant);''', '''    // 🔒️ Per-connection `SecurityGate` compiled from the hub's one declared access policy: this
    // connection's roles read and write exactly what `document.read`/`document.write` permit in
    // this space's kind (an archive denies every write). A space whose kind cannot be read compiles
    // to no grant at all. `TenantId` reuses the space id: every scope this gate ever evaluates
    // belongs to exactly this one space/document connection.
    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map(|space| space.kind);
    let access_roles = auth.access_roles();
    let granted: Vec<db::security::Action> = [(HubAccessActionV1::DocumentRead, db::security::Action::Read), (HubAccessActionV1::DocumentWrite, db::security::Action::Write)]
        .into_iter()
        .filter(|(action, _)| space_kind.as_deref().is_some_and(|kind| hub_access_permits(&access_roles, *action, Some(kind))))
        .map(|(_, granted)| granted)
        .collect();''')
rep('''    let principal = db::security::Principal::new(actor.clone(), tenant.clone(), vec![role_str]);''', '''    let policy = db::security::RoleBasedPolicy::new().with_grant(db::security::Grant::allow(role_str.clone(), &["db", "document", "*", "**"], &granted));
    let principal = db::security::Principal::new(actor.clone(), tenant.clone(), vec![role_str]);''')

# check-in author: declared policy in the document's space
rep('''async fn check_in_author(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, String), StatusCode> {
    let (subject, _) = authenticate_document_socket_subject(state, scope, headers).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match &subject {
        SocketSubjectV1::Session { role: Some(SpaceRole::Author), user_id, .. } => {
            let user_id = user_id.clone();
            Ok((subject, user_id))
        }
        SocketSubjectV1::Session { .. } => Err(StatusCode::FORBIDDEN),
        SocketSubjectV1::Share { .. } => Err(StatusCode::UNAUTHORIZED),
    }
}''', '''async fn check_in_author(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, String), StatusCode> {
    let (subject, _) = authenticate_document_socket_subject(state, scope, headers).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    let SocketSubjectV1::Session { user_id, .. } = &subject else { return Err(StatusCode::UNAUTHORIZED) };
    let user_id = user_id.clone();
    if !access_permits_in_space(state, &subject.access_roles(), HubAccessActionV1::DocumentCheckIn, &scope.space_id).await {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok((subject, user_id))
}''')

# artifact creation: final commit authority + route admission
rep('''            match tokio::time::timeout(std::time::Duration::from_secs(2), self.directory.socket_session_binding(&actor.session_id, &actor.user_id, actor.authorization_generation, Some(space_id), now_ms())).await {
                Ok(Ok(SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), .. })) => Ok(Box::new(HubArtifactCreationCommitLeaseV1 { _guards: guards }) as Box<dyn ArtifactCreationCommitLeaseV1>),''', '''            let space_kind = self.directory.get_space(space_id).await.ok().flatten().map(|space| space.kind);
            match tokio::time::timeout(std::time::Duration::from_secs(2), self.directory.socket_session_binding(&actor.session_id, &actor.user_id, actor.authorization_generation, Some(space_id), now_ms())).await {
                Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), .. })) if space_kind.as_deref().is_some_and(|kind| hub_access_permits(&[HubAccessRoleV1::Authenticated, space_role_access(role)], HubAccessActionV1::ArtifactCreate, Some(kind))) => {
                    Ok(Box::new(HubArtifactCreationCommitLeaseV1 { _guards: guards }) as Box<dyn ArtifactCreationCommitLeaseV1>)
                }''')
rep('''    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), .. })) => Ok((ArtifactCreationActorV1 { user_id: caller.user_id, session_id: caller.session_id, authorization_generation: caller.authorization_generation }, guards)),''', '''    let space_kind = state.directory.get_space(space_id).await.ok().flatten().map(|space| space.kind);
    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), .. })) if space_kind.as_deref().is_some_and(|kind| hub_access_permits(&[HubAccessRoleV1::Authenticated, space_role_access(role)], HubAccessActionV1::ArtifactCreate, Some(kind))) => {
            Ok((ArtifactCreationActorV1 { user_id: caller.user_id, session_id: caller.session_id, authorization_generation: caller.authorization_generation }, guards))
        }''')

# directory commands
rep('''async fn authorize_directory_command(state: &HubState, actor_user_id: &str, admin: bool, command: &DirectoryCommand) -> Result<(), StatusCode> {
    if admin {
        return Ok(());
    }
    match command {
        DirectoryCommand::CreateSpace { .. } => Ok(()),
        DirectoryCommand::DeleteSpace { space_id } | DirectoryCommand::ArchiveSpace { space_id } => {
            let space = state.directory.get_space(space_id).await.map_err(directory_error_status)?.ok_or(StatusCode::NOT_FOUND)?;
            if space.owner_user_id == actor_user_id {
                Ok(())
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => match state.directory.get_role(space_id, actor_user_id).await {
            Ok(Some(SpaceRole::Author)) => Ok(()),
            Ok(_) => Err(StatusCode::FORBIDDEN),
            Err(error) => Err(directory_error_status(error)),
        },
        DirectoryCommand::AnnounceDocument { descriptor } => match state.directory.get_role(&descriptor.space_id, actor_user_id).await {
            Ok(Some(SpaceRole::Author)) => Ok(()),
            Ok(_) => Err(StatusCode::FORBIDDEN),
            Err(error) => Err(directory_error_status(error)),
        },
    }
}''', '''/// 🎬️ The declared-policy action one directory command asks for.
fn directory_command_access_action(command: &DirectoryCommand) -> HubAccessActionV1 {
    match command {
        DirectoryCommand::CreateSpace { .. } => HubAccessActionV1::SpaceCreate,
        DirectoryCommand::RenameSpace { .. } => HubAccessActionV1::SpaceRename,
        DirectoryCommand::SetVisibility { .. } => HubAccessActionV1::SpaceVisibility,
        DirectoryCommand::ArchiveSpace { .. } => HubAccessActionV1::SpaceArchive,
        DirectoryCommand::DeleteSpace { .. } => HubAccessActionV1::SpaceDelete,
        DirectoryCommand::UpsertMember { .. } => HubAccessActionV1::MemberUpsert,
        DirectoryCommand::RemoveMember { .. } => HubAccessActionV1::MemberRemove,
        DirectoryCommand::CreateInvite { .. } => HubAccessActionV1::InviteCreate,
        DirectoryCommand::RevokeInvite { .. } => HubAccessActionV1::InviteRevoke,
        DirectoryCommand::AnnounceDocument { .. } => HubAccessActionV1::DocumentAnnounce,
    }
}

/// 🛡️ A directory command is admitted exactly when the declared access policy permits its action to
/// the actor's roles in the command's space: operator subject, space owner, membership role, and
/// being signed in at all. A space-scoped command naming a space that does not exist is `404`.
async fn authorize_directory_command(state: &HubState, actor_user_id: &str, admin: bool, command: &DirectoryCommand) -> Result<(), StatusCode> {
    let mut roles = vec![HubAccessRoleV1::Authenticated];
    if admin {
        roles.push(HubAccessRoleV1::Admin);
    }
    let space_kind = match directory_command_space(command) {
        None => None,
        Some(space_id) => {
            let space = state.directory.get_space(space_id).await.map_err(directory_error_status)?.ok_or(StatusCode::NOT_FOUND)?;
            if space.owner_user_id == actor_user_id {
                roles.push(HubAccessRoleV1::Owner);
            }
            if let Some(role) = state.directory.get_role(space_id, actor_user_id).await.map_err(directory_error_status)? {
                roles.push(space_role_access(role));
            }
            Some(space.kind)
        }
    };
    if hub_access_permits(&roles, directory_command_access_action(command), space_kind.as_deref()) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}''')
open(p, "w", encoding="utf-8").write(s)
print("pass2 ok")

p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🦀️.rs"
s = open(p, encoding="utf-8").read()
start = s.index("//#region 🔖️SpaceGrants\n")
end = s.index("//#endregion 🔖️SpaceGrants\n") + len("//#endregion 🔖️SpaceGrants\n")
s = s[:start] + s[end:]
if s[start:start+1] == "\n":
    s = s[:start] + s[start+1:]
open(p, "w", encoding="utf-8").write(s)
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🧪️tests/🔬️unit/🦀️.rs"
s = open(p, encoding="utf-8").read()
start = s.index("//#region 🔖️SpaceGrants\n")
end = s.index("//#endregion 🔖️SpaceGrants\n") + len("//#endregion 🔖️SpaceGrants\n")
s = s[:start] + s[end:]
if s[start:start+1] == "\n":
    s = s[:start] + s[start+1:]
open(p, "w", encoding="utf-8").write(s)
print("db ok")
