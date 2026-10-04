#!/usr/bin/env python3
"""🏪️ S3-CLOSURE wave 6a/6b on `🏪️store/🦀️.rs`: deletes `ArtifactCommand::AmendLast*` (+ text/binary codecs, `amend_command`),
the coalesce branch of `batch_amend_target`, `ArtifactStoreBatchPublication.coalesce_key`/`set_coalesce_key`, every
`Edit.coalesce_key` read/write (retirement, owned `.spr` decoder, `.ops` header, history projection, digest) and
`GroupMeta.coalesce_key`. Every replacement asserts its exact anchor count, so a drifted file is refused untouched."""
import sys
P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
R = [
("        let Edit { id, actor, line, forwards, inverse, mutation_meta, description, verb, coalesce_key, sequence_number: _, started_at, finished_at } = edit;",
 "        let Edit { id, actor, line, forwards, inverse, mutation_meta, description, verb, sequence_number: _, started_at, finished_at } = edit;"),
("strings: [Some(id), actor, description, verb, coalesce_key, Some(started_at), finished_at, line]", "strings: [Some(id), actor, description, verb, Some(started_at), finished_at, line]"),
("struct ArtifactStoreEditRetirementState {\n    strings: [Option<String>; 8],", "struct ArtifactStoreEditRetirementState {\n    strings: [Option<String>; 7],"),
("""    /// 🛤️ `AmendLast`'s lane-tagged twin — see `ApplyInLane`'s doc for why this is an
    /// additive new variant rather than a field on `AmendLast`.
    AmendLastInLane {
        mutations: Vec<Mutation>,
        coalesce_key: Option<String>,
        #[value(default)]
        lane: HistoryLane,
    },
""", ""),
("""    AmendLast {
        mutations: Vec<Mutation>,
        /// 🪢️ Matches the last uncommitted edit's `coalesce_key` to absorb into it instead of creating a new edit.
        coalesce_key: Option<String>,
    },
""", ""),
("            Self::Apply { .. } | Self::ApplyInLane { .. } | Self::AmendLast { .. } | Self::AmendLastInLane { .. } => Some(ArtifactProjectionCause::Apply),",
 "            Self::Apply { .. } | Self::ApplyInLane { .. } => Some(ArtifactProjectionCause::Apply),"),
("            | Self::ApplyInLane { .. }\n            | Self::AmendLast { .. }\n            | Self::AmendLastInLane { .. }\n", "            | Self::ApplyInLane { .. }\n"),
("    OwnedSchemaFieldSpec { id: 7, key: \"coalesceKey\", required: false },\n", ""),
("    strings: [std::mem::ManuallyDrop<Option<String>>; 8],", "    strings: [std::mem::ManuallyDrop<Option<String>>; 7],"),
("            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 6 | 7 | 10 | 11 | 12) {", "            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 6 | 10 | 11 | 12) {"),
("""            1 => Some(0),
            2 => Some(1),
            6 => Some(2),
            7 => Some(3),
            9 => Some(4),
            10 => Some(5),
            11 => Some(6),
            12 => Some(7),
            _ => None,""", """            1 => Some(0),
            2 => Some(1),
            6 => Some(2),
            9 => Some(3),
            10 => Some(4),
            11 => Some(5),
            12 => Some(6),
            _ => None,"""),
("        let started_at = self.strings[4].take().ok_or_else(|| self.diagnostic(\"artifact-spr.edit-started-at-missing\", 0))?;",
 "        let started_at = self.strings[3].take().ok_or_else(|| self.diagnostic(\"artifact-spr.edit-started-at-missing\", 0))?;"),
("Some(Edit { id, actor: self.strings[1].take(), line: self.strings[7].take(), forwards, inverse, mutation_meta: Vec::new(), description: self.strings[2].take(), verb: self.strings[6].take(), coalesce_key: self.strings[3].take(), sequence_number, started_at, finished_at: self.strings[5].take() });",
 "Some(Edit { id, actor: self.strings[1].take(), line: self.strings[6].take(), forwards, inverse, mutation_meta: Vec::new(), description: self.strings[2].take(), verb: self.strings[5].take(), sequence_number, started_at, finished_at: self.strings[4].take() });"),
("""/// Called from every fresh-edit constructor (`apply_command`, `amend_command`'s fresh branch); an
/// `AmendLast` extension always appends beyond index 0 of an already multi-op edit, so it never
/// qualifies.""", """/// Called from every fresh-edit constructor (`apply_command`); a streamed transaction tick always
/// appends beyond index 0 of its open edit, so it never qualifies."""),
("        finished: edit.finished_at.clone(),\n        key: edit.coalesce_key.clone(),\n        description: edit.description.clone(),", "        finished: edit.finished_at.clone(),\n        description: edit.description.clone(),"),
("        finished_at: edit.finished_at.clone(),\n        coalesce_key: edit.coalesce_key.clone(),\n        description: edit.description.clone(), verb: edit.verb.clone(), line: edit.line.clone(),",
 "        finished_at: edit.finished_at.clone(),\n        description: edit.description.clone(), verb: edit.verb.clone(), line: edit.line.clone(),"),
("            description: history_edit.description, verb: history_edit.verb, line: history_edit.line,\n            coalesce_key: history_edit.coalesce_key,\n",
 "            description: history_edit.description, verb: history_edit.verb, line: history_edit.line,\n"),
("        finished_at: Option<String>,\n        coalesce_key: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }",
 "        finished_at: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }"),
("            description: header.description, verb: header.verb, line: header.line,\n            coalesce_key: header.coalesce_key,\n",
 "            description: header.description, verb: header.verb, line: header.line,\n"),
("            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, key, description, verb, line } => {",
 "            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, description, verb, line } => {"),
("finished_at: finished, coalesce_key: key, description, verb, line: ops_line_provenance(line)? });", "finished_at: finished, description, verb, line: ops_line_provenance(line)? });"),
("        actor: Option<String>,\n        finished: Option<String>,\n        key: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Vec<String>,\n    },",
 "        actor: Option<String>,\n        finished: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Vec<String>,\n    },"),
("""/// `create-alternative`/`switch-alternative`/`checkout`/`amend` — the command-level twin of
/// `OpsHeaderLine`, re-derived on the same `dsl_schema` grammar engine. `Apply`/`Amend` carry no""", """/// `create-alternative`/`switch-alternative`/`checkout` — the command-level twin of
/// `OpsHeaderLine`, re-derived on the same `dsl_schema` grammar engine. `Apply` carries no"""),
("""    Amend {
        key: Option<String>,
    },
    PruneDrafts,""", """    PruneDrafts,"""),
("""    AmendInLane {
        key: Option<String>,
        lane: String,
    },
""", ""),
("/// 🛤️ `HistoryLane`'s text token — the `ApplyInLane`/`AmendInLane`/`UndoInLane`/", "/// 🛤️ `HistoryLane`'s text token — the `ApplyInLane`/`UndoInLane`/"),
("/// 2-space-indented operation lines (`Apply`/`AmendLast`) or a further-indented nested command", "/// 2-space-indented operation lines (`Apply`) or a further-indented nested command"),
("""        ArtifactCommand::AmendLast { mutations, coalesce_key } => {
            out.push_str(&CommandHeaderLine::Amend { key: coalesce_key.clone() }.print_op());
            out.push('\\n');
            print_indented_ops(&mut out, mutations).await?;
        }
""", ""),
("""        ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => {
            out.push_str(&CommandHeaderLine::AmendInLane { key: coalesce_key.clone(), lane: history_lane_to_token(*lane).await.to_string() }.print_op());
            out.push('\\n');
            print_indented_ops(&mut out, mutations).await?;
        }
""", ""),
("""        CommandHeaderLine::Amend { key } => {
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend requires at least one operation line"));
            }
            Ok(ArtifactCommand::AmendLast { mutations, coalesce_key: key })
        }
""", ""),
("""        CommandHeaderLine::AmendInLane { key, lane } => {
            let lane = parse_history_lane_token(&lane).await?;
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend-in-lane requires at least one operation line"));
            }
            Ok(ArtifactCommand::AmendLastInLane { mutations, coalesce_key: key, lane })
        }
""", ""),
("        ArtifactCommand::Apply { mutations, .. } | ArtifactCommand::ApplyInLane { mutations, .. } | ArtifactCommand::AmendLast { mutations, .. } | ArtifactCommand::AmendLastInLane { mutations, .. } | ArtifactCommand::AppendTransaction { mutations, .. } => {",
 "        ArtifactCommand::Apply { mutations, .. } | ArtifactCommand::ApplyInLane { mutations, .. } | ArtifactCommand::AppendTransaction { mutations, .. } => {"),
("""            ArtifactCommand::AmendLast { mutations, coalesce_key } => {
                crate::os_pack::write_varint_u64(&mut out, 8);
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key);
                }
                write_command_ops(&mut out, mutations)?;
            }
""", ""),
("""            ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => {
                crate::os_pack::write_varint_u64(&mut out, 12);
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key);
                }
                out.push(history_lane_ordinal(*lane));
                write_command_ops(&mut out, mutations)?;
            }
""", ""),
("""            8 => {
                let presence = reader.read_u8()?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::AmendLast { mutations, coalesce_key })
            }
""", ""),
("""            12 => {
                let presence = reader.read_u8()?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                let lane = history_lane_from_ordinal(reader.read_u8()?)?;
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane })
            }
""", ""),
("""                &tag(edit.coalesce_key.as_ref()),
                edit.coalesce_key.as_deref().unwrap_or_default().as_bytes(),
""", ""),
("&mut self.edit.description, &mut self.edit.verb, &mut self.edit.coalesce_key, &mut self.edit.finished_at,", "&mut self.edit.description, &mut self.edit.verb, &mut self.edit.finished_at,"),
("            && self.edit.verb.is_none()\n            && self.edit.coalesce_key.is_none()\n", "            && self.edit.verb.is_none()\n"),
("    phase: ArtifactStoreOneItemPublicationPhase,\n    coalesce_key: Option<String>,\n    verb: Option<String>,", "    phase: ArtifactStoreOneItemPublicationPhase,\n    verb: Option<String>,"),
("""    /// 🎥️ Latest-wins undo key for this batched gesture. When it matches the tail uncommitted
    /// document edit, commit amends that edit (one undo step) instead of minting a ledger slot.
    pub fn set_coalesce_key(&mut self, key: Option<String>) {
        self.coalesce_key = key.filter(|value| !value.is_empty());
    }

""", ""),
("        if self.coalesce_key.take().is_some() || self.verb.take().is_some() {", "        if self.verb.take().is_some() {"),
("        } else if self.coalesce_key.is_some() {\n            \"coalesce-key\"\n", ""),
("""    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries,
    /// else the tail uncommitted document edit its coalesce key matches, as `amend_command` does.
    fn batch_amend_target(&self, key: Option<&str>, transaction: Option<&protocol::TransactionRef>) -> Option<String> {
        if let Some(transaction) = transaction {
            return self.envelope.open_transaction.as_ref().filter(|open| open.transaction.id == transaction.id && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone());
        }
        let key = key.filter(|value| !value.is_empty())?;
        let last = self.applied_edit_ids.last()?;
        let edit = self.envelope.vcs.edits.iter().find(|edit| edit.id == *last)?;
        if edit.coalesce_key.as_deref() != Some(key) {
            return None;
        }
        let committed = self.envelope.vcs.changes.iter().any(|change| change.edit_ids.iter().any(|id| id == last));
        if committed {
            return None;
        }
        Some(last.clone())
    }""", """    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries
    /// (design §15). Outside a transaction every batch is its own edit.
    fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone())
    }"""),
("self.batch_amend_target(publication.coalesce_key.as_deref(), publication.transaction.as_ref()).is_some();", "self.batch_amend_target(publication.transaction.as_ref()).is_some();"),
("if let Some(edit_id) = self.batch_amend_target(publication.coalesce_key.as_deref(), publication.transaction.as_ref()) {", "if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {"),
("                        return Err(VcsError::ValidationFailed(\"batched amend lost its coalesced tail edit\".into()));", "                        return Err(VcsError::ValidationFailed(\"batched transaction append lost its open edit\".into()));"),
("            stage.edit.coalesce_key = publication.coalesce_key.clone();\n", ""),
("""            ArtifactCommand::AmendLast { mutations, coalesce_key } => self.amend_command(mutations, coalesce_key, HistoryLane::Document).await,
            ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => self.amend_command(mutations, coalesce_key, lane).await,
""", ""),
("""/// 🎛️ Cross-member metadata for one `dispatch_group` call. `description` becomes every
/// dispatched member's own `ArtifactCommand::Apply.description`. `actor`/`coalesce_key` are
/// accepted or forward-compat/audit purposes but NOT yet wired into dispatch — object-safe
/// `SpaceMember` has no `set_local_actor_id`/`AmendLast` seam today (only `ArtifactStore`'s own
/// inherent API does), so honoring them is deferred to whichever later wave extends that surface;
/// see `📓️wave1-reports/b2-store-composition-report.md`'s scoping note.""", """/// 🎛️ Cross-member metadata for one `dispatch_group` call. `description` becomes every
/// dispatched member's own `ArtifactCommand::Apply.description`; `actor` is recorded for audit but not wired into
/// dispatch (object-safe `SpaceMember` has no `set_local_actor_id` seam; only `ArtifactStore`'s inherent API does)."""),
("    pub description: Option<String>,\n    pub coalesce_key: Option<String>,\n    /// 🪪️ The group identity every member's tail edit", "    pub description: Option<String>,\n    /// 🪪️ The group identity every member's tail edit"),
]
def apply(text):
    for a, b in R:
        count = text.count(a)
        if count != 1:
            sys.exit(f"anchor count {count} != 1: {a[:90]!r}")
        text = text.replace(a, b)
    return text
if __name__ == "__main__":
    src = open(P, encoding="utf-8").read()
    out = apply(src)
    start = out.index("    /// 🛤️ Shared body of `AmendLast`/`AmendLastInLane`")
    end = out.index("    //#endregion 🔖️HistoryLane", start)
    out = out[:start].rstrip(" ") + out[end:]
    open(P, "w", encoding="utf-8").write(out)
    print("ok")
