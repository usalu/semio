# Global Configuration Field Owner Refresh

Root repeated a focused live-source scan under `✏️s/🔌️plugins` on 2026-09-13 for locale, language, theme, timezone, keyboard, shortcut, client/user identity, font, unit-system, privacy, telemetry and accessibility names within config owners. This is a candidate scan, not a complete semantic classification of all fields.

No matching named mutation/config leaf appeared under the config directory suffixes. A content scan of config Rust schemas and JSON fields found the shared Space engine `client_id/client_name` and the Home read-model `userId` member fields. Expanding to all config Rust source also found SpaceIndexMember.user_id and Writer's exact-window font_px.

The Space engine identity pair is being traced by the Home/Space execution agent alongside the confirmed Home OS-session identity mirror. It must use the same host-owned current authenticated identity if it has that meaning. Source location: `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/🧬️schema/🦀️.rs` and its aggregate/other schema facets.

SpaceIndexMember.user_id is explicitly a member record projected from the OS Directory read model; it is not itself the current authenticated user preference. It must not be mechanically removed based on the field name. Its surrounding persisted app read-model/presence lifecycle remains part of the separately tracked directory/read-model ownership review. Writer font_px is declared under the concrete main window config and is likewise not evidence of a global OS setting.

The original Jack locale leaf remains absent in the previously accepted broad ownership gate. This focused scan made no production edits and ran no native test. It does not supersede the producer/consumer findings in `audit-remaining-app-config-terra.md` or the remaining runtime and persistence requirements in the completion checklist.

The first filename query accidentally matched the absolute `/Users` prefix and was discarded; the corrected query anchors matching after the config directory. Generated command output is kept only under the ticket's generated folder.
