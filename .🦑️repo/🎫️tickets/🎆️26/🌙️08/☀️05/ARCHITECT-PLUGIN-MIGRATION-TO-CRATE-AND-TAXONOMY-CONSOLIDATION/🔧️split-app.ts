#!/usr/bin/env bun
/** 🎛️ One-shot app-side extraction: pulls the old 🛂️manifest bundle crate's line ranges into the new
 *  `🎛️apps/🏛️architect/**` taxonomy files, rewriting the old spine crate paths onto the artifact tree
 *  and making the moved free functions `pub`. Scratch tool for ticket
 *  `26/08/05/ARCHITECT-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginRoot = join(repoRoot, "✏️s/🔌️plugins/🏛️architect");
const bundle = join(pluginRoot, "🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs");
const lines = readFileSync(bundle, "utf8").split("\n");

/** ✂️ 1-based inclusive line range. */
function slice(from: number, to: number): string {
  return lines.slice(from - 1, to).join("\n");
}

/** 🧹 Repoints the ex-spine crate paths and promotes moved private fns/consts to `pub`. */
function rewrite(text: string): string {
  return text
    .replace(/semio_s_plugin_architect_spine::/g, "crate::artifacts::program::")
    .replace(/^fn /gm, "pub fn ")
    .replace(/^const /gm, "pub const ")
    .replace(/^struct /gm, "pub struct ");
}

function emit(relPath: string, header: string, body: string) {
  const abs = join(pluginRoot, relPath);
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, `${header}\n${body}\n`);
  console.log(`wrote ${relPath} (${body.split("\n").length} body lines)`);
}

const APP = "🎛️apps/🏛️architect";

const catalogBody = [
  slice(38, 105), // REGISTER_IDS
  slice(272, 352), // next_adjacency_kind .. parse_entity_id_from_args
  slice(357, 910), // register_entities .. report_record_from
  slice(1016, 1139), // parse_entity_id .. report_kind_picker_options
].join("\n\n");

emit(
  `${APP}/🦀️catalog.rs`,
  `//! 🗂️ Architect play app — the register catalog: which registers exist, how to enumerate and
//! inspect their rows, how to build the add/remove/patch operations for them, and how to coerce the
//! host's stringly action args into the artifact's typed kinds.
//!
//! App level (not artifact engine) on purpose: every function here exists to serve the app's command
//! and panel layers, several produce framework \`ActionArgOption\`s, and the artifact has no other
//! consumer that would benefit from owning them.
`,
  rewrite(catalogBody),
);

emit(
  `${APP}/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️component.rs`,
  "//! ↔️ Architect adjacency window — the signature adjacency matrix surface.\n",
  rewrite(slice(1150, 1221)),
);

emit(
  `${APP}/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs`,
  "//! 🕸️ Architect graph window — the element/adjacency node-graph surface.\n",
  rewrite(slice(1225, 1281)),
);

emit(`${APP}/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️component.rs`, "//! 📋️ Architect register window — the active register's rows as a block-list surface.\n", rewrite([slice(219, 232), slice(1285, 1311)].join("\n\n")));

emit(`${APP}/🎭️modes/✏️edit/🪟️windows/📄️report/🦀️component.rs`, "//! 📄️ Architect report window — the last generated `ProgramReport`, rendered as a section tree.\n", rewrite(slice(1380, 1407)));

emit(`${APP}/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️component.rs`, "//! 🧭️ Architect trace window — trace chain, impact and audit trail for the selected entity.\n", rewrite(slice(1409, 1429)));

emit(`${APP}/📌️panels/📄️document/🦀️component.rs`, "//! 📄️ Architect document panel — program meta, register counts and the element list.\n", rewrite(slice(1316, 1351)));

emit(`${APP}/📌️panels/📚️catalogue/🦀️component.rs`, "//! 📚️ Architect catalogue panel — the action shortcuts and the register index.\n", rewrite(slice(1353, 1378)));

emit(`${APP}/📌️panels/🔍️inspection/🦀️component.rs`, "//! 🔍️ Architect inspection panel — typed inspectors for the selected entity.\n", rewrite(slice(1431, 1495)));
