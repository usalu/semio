import { readdirSync, readFileSync, writeFileSync, statSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = join(ticket, "../../../../../..");
const ticketsRoot = join(root, ".🦑️repo/🎫️tickets");

function walk(d, acc = []) {
  for (const e of readdirSync(d)) {
    const p = join(d, e);
    try {
      if (statSync(p).isDirectory()) walk(p, acc);
      else if (e === "🎫️ticket.json") acc.push(p);
    } catch {}
  }
  return acc;
}

const open = [];
for (const p of walk(ticketsRoot)) {
  try {
    const t = JSON.parse(readFileSync(p, "utf8"));
    if (t.status === "open") open.push(`- ${p.split("/").slice(-2, -1)[0]}: ${t.title}`);
  } catch {}
}

writeFileSync(
  join(ticket, "collision-map-v2.txt"),
  `# Collision map v2 ${new Date().toISOString()}
## Dirty plugins
✏️s/🔌️plugins/🏭️process
✏️s/🔌️plugins/💠️lowpoly
## Open tickets (${open.length})
${open.join("\n")}
## Hot plugins (defer to W5f)
🌊️flow, 🌀️procedural, 🧱️block, � contpuzzle, 🌿️vcs
## Pilots (P4 exclusive)
💠️lowpoly, 📕️norm/📘️en1992, 🕸️dag, 📐️cad
`
);

writeFileSync(
  join(ticket, "protocol-dialect-contract-v2.md"),
  `# Protocol dialect contract v2

## Model
ProtocolFile { id, version, schema, start, uses, framing, blocks }
Framing = Magic([u8;8]) | Record | Chunked
Block = Header | Segment | Record | Struct | Enum | Footer | Chain
Prim = U8|U16|U32|U64|I32|I64|F32|F64|Varint|Zigzag|Bytes|Utf8|Fixed(n)|Array|Ref

## Laws
1. parse_protocol retains every directive (no skip_line).
2. print_protocol(parse_protocol(x)) round-trips body; canonicalize is idempotent.
3. walk_protocol(spec, bytes) consumes exactly bytes.len() or returns ProtocolMismatch with offset.
4. use loads shared struct/enum fragments; local shadows fragment.
5. Specs are normative-and-verified, not codegen. Encoders stay handcrafted Rust.
`
);

writeFileSync(
  join(ticket, "distinctness-contract-v2.md"),
  `# Distinctness contract v2

## Policy breaches (high priority)
1. policySpecDistinctnessBreaches — normalized hash collision
2. policyGenericSpecBreaches — catch-all prop/list/map/value, bare IDENT assign*, json/blob fields
3. policyDeclaredUseBreaches — use family-X must reference fragment production
4. policySpecWiringBreaches — include_str! + register_language
5. policyEmptyExampleBreaches — pack/spr examples exceed SEM envelope

## Sweep laws
Grammar conformance, production coverage, protocol walk, cross-artifact rejection.
`
);

writeFileSync(
  join(ticket, "family-notation-guide-v2.md"),
  `# Family notation guide v2

| Family | Artifacts | Key terminals |
|--------|-----------|---------------|
| F1 graph | dag, wires, jack, rewrite, flow, sequence | node, port, ARROW/EDGEARROW, chain |
| F2 mesh | lowpoly, procedural*, block*, puzzle*, cad, process3d, remodel | VEC3, halfedge, face, transform |
| F3 sheet | 15 norms, architect, energy | QUANTITY=FLOAT UNIT, clause, verdict |
| F4 canvas | draw, raster, note, layout, present, shooting, forms | stroke, layer, COLOR, box |
| F5 catalog | curate, home, playground | stock, typology, compat |
| F6 text | writer, imperative, playbook, mathematical | statement, fence, expr |
| F7 geo | gismap, gisterrain | POINT, CRS, tile |
| F8 eng | fem2d, fem3d, vcs | node, element, load, support, commit |

Per-artifact 4-byte domain magic + segment kinds + one spr record tag per Operation variant.
`
);

writeFileSync(
  join(ticket, "verification-checklist-v2.md"),
  `# Verification checklist v2

- [ ] Protocol dialect self-hosts
- [ ] walk_protocol proven on real pack+spr
- [ ] Recognizer resolves use family-*; BOOL/arrows match
- [ ] Five policy rules armed
- [ ] Fixture sweep: grammar + coverage + protocol + cross-reject
- [ ] Pilots: lowpoly, en1992, dag, cad
- [ ] Fan-out W5a-W5f complete
- [ ] Derive emission deleted
- [ ] policy + test dsl + verify green
- [ ] Writer opens 6+ kinds; ticket close
`
);

writeFileSync(
  join(ticket, "wave-ownership-v2.txt"),
  `# Wave ownership v2

P4 pilots: lowpoly | norm/en1992 | dag | cad

W5a F1: reasoning trinity sequence mathematical (flow if free)
W5b F3: norm(remaining) architect energy
W5c F4: draw raster note layout animate shooting forms playbook
W5d F5+F6+F7: sourcing space demonstrator writer imperative gis
W5e F8+F2 mid: fem process remodel
W5f HOT: flow procedural block puzzle vcs

Orchestrator-only: script.ts Cargo.toml package.json launch.json
Engine freeze after P1: dsl/** pack/** spr/** store/**
`
);

writeFileSync(join(ticket, "engine-requests.txt"), `# Engine hotfix queue (drained between waves)\n`);
writeFileSync(join(ticket, "progress-v2.md"), `# Progress v2\n\n- P0 bootstrap: contracts + collision + ownership written\n`);

console.log("P0 written", ticket, "open=", open.length);
