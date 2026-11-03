# No defaults (wave 2, 2026-11-03)

Nothing compiled; checked by reading. Rebuild requested ("ready for rebuild: no defaults" in build-en1990..en1999 latest.md).

## Check 27, choice-typed defaults
- En1990Snapshot, En1991Snapshot: `impl Default` removed; `example(annex: AnnexChoice)` constructor. 178 call sites in 52 files now `::example(AnnexChoice::De)` (short or fully qualified `semio_s_plugin_norm_compliance::AnnexChoice`; claims test in en1991 uses `En`). Also `ArtifactBuilder::empty`, `reference_birth`, io, case-edits tests (`assert_case_edit_paths` seed), `norm_latency_bench!(seed = ..)`.
- Shared: `NormFamily::Document` lost `Default`; `NormHost<F>: Default` only where the document is `Default` (din4108 test uses it). Latency bench takes an explicit seed (`seed = <expr>` macro arm; default arm via `default_seed::<A>()` for other artifacts).
- En1990Outline / En1991Outline keep `Default` (required by `Inference: Default`), computed from `example(De)`; carries only counts and clause labels, no choice.
- MasonryClass (en1996): `Default`/`#[default]` removed. No other user.
- RebarProduct (en1994): `Default` removed. Real bug found: `unwrap_or_default()` silently mapped unknown products to hot-rolled (column check, beam fire). Now `beam_fire` returns `Option<Beam>`; compliance (column and beam), scene projection and diagrams report NotApplicable or skip; test updated to expect None.
- SectionMethod (mechanics): `Default` removed; `SectionConstants` lost `Default` too, `analyse_thin_walled` lists all fields.
- ReliabilityClass: left for RC-THREADING v2, noted in requests.md.
- Reviewed, kept: SectionOptions (mesh tuning), Tier and InspectorOptions (UI preferences, user-customizable), ViewOptions (Scope::All), ProfiledSheeting, TorsionProfile, GroundInputs, CaseValues, mechanics Member/LoadCase/Model/MemberForces/BucklingMode/ModalMode/LtbMember, en1999 patches (all zero, empty or `Option` "not given" values, no normative choice).
- Open (reported, not changed, outside brief): En1992/1993/1994/1995/1996/1997/1998/1999 Snapshot `impl Default` still delegate to named examples with annex De.

## Check 1, path at 256
- Kinds renamed in all layers: `change-column-reinforcement-product` -> `change-column-rebar-product`, `change-beam-reinforcement-product` -> `change-beam-rebar-product` (matches the beam leaf EN 1994 v10 was adding; sibling kinds already use `rebar`). Folders (schema and fixtures), rs/ts/json/proto/graphql/protocol, taxonomy fixture, ticket generator scripts. Fields `reinforcementProduct` unchanged.
- Longest norm path now 255 B (an en1998 kind, early warning), none >= 256; en1994 longest 248 B.

## Risks
- Kind order in sorted goldens/lists may change (`rebar` sorts differently); fixtures may need regeneration if a golden enumerates kinds.
- En1994 v10 edited the same area concurrently (beam leaf); coordinated via requests.md.
