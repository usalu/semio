# C4d protocol change — document WS identity

## Breaking change (landed)

`GET /scopes/{scope}/document/ws` no longer accepts identity-bearing query parameters.

**Removed from the protocol:**
- `actor` — was caller-supplied; any subscriber could impersonate
- `session` — was caller-supplied presence/session id

**Retained (non-identity):**
- `surface`
- `resume`

## Server-side identity

Socket actor and session id are derived only from the authenticated principal via `DocumentAuthority::bind_socket` / `Resolved.actor` (hub: `document_actor_id(session.secret_digest, true)` from the `semio.session.v1` credential). Clients MUST NOT send actor/session in the URL.

## Peers

- O3b / C4c / M10b: drop `?actor=` when opening the document WS; keep session credential auth. M10b `[DEBUG] m10b` logs left untouched.
