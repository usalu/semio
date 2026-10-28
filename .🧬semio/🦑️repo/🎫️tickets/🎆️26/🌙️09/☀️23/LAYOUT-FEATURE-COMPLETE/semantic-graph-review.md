# Semantic Graph Review

Read-only native mutation audit against the canonical validator. No Rust compilation or native test pass is claimed. The existing language-neutral validation fixture is the common valid base below. Its independent Ajv/topological oracle reported baseValid=true and resultValid=false for all nine post-edit witnesses listed below. These are projections of the actual sparse diffs, not execution of the native mutations.

## Shared Base and Writer Evidence

Base: C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/✅️validation/🔣️.json, property base. It has page-1 in spread-1, text-1 -> text-2, story-1 content Aé🌍 with UTF-8 run [1,7], and inherited parent-frame. All unrelated rows stay as authored in that fixture.

Writer: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️diff-apply-and-algebra/🦀️.rs:40-96 applies sparse list changes and installs the prepared graph without calling validate_document. ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️row-patches/🦀️.rs:298-337 derives page layer memberships after frame updates. Consequently stale layer objectIds after frame insertion/removal are NOT findings. It does not derive spread membership or repair thread references.

## P1: Updating a Thread Can Close a Cycle

Source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️update-text-frame/🦀️.rs:65-69 and 76-89.

Smallest payload on the shared base: {pageId: "page-1", frameId: "text-2", storyId: "story-1", threadNext: "text-1", insetX: 0, insetY: 0, insetWidth: 0, insetHeight: 0}. The guard rejects self-targets and missing/non-text targets, but text-1 is an existing distinct text frame. It emits text-2 -> text-1 alongside existing text-1 -> text-2. Canonical rejection: ✅️validation/🦀️.rs:63. Missing targets are already guarded; the established defect is transitive cycles.

Fix: evaluate the candidate outgoing edge through the existing thread validator or walk the target chain before producing the diff. Test a two-frame and cross-page cycle, plus allowed cross-page acyclic threading.

## P1: Deleting a Thread Target Leaves an Inbound Edge

Source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-frame/🦀️.rs:49-59.

Payload: {pageId: "page-1", frameId: "text-2"}. It emits only the frame removal; text-1.threadNext remains text-2. Derived layer membership correctly becomes [text-1], but the thread target no longer exists. Canonical rejection: ✅️validation/🦀️.rs:54.

Fix: either refuse deletion while referenced, or clear/reconnect every affected inbound edge in the same diff, retaining each original edge for inverse/undo. Include cross-page inbound edges.

## P1: Text Editing Retains Invalid Byte Runs

Source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✍️edit-story/🦀️.rs:62-76; explicitly style_runs: None at line 74.

Payload A: {id: "story-1", offset: 0, delete: 3, insert: ""}. The splice is within the three-character story, but the output content is empty while the run stays [1,7]. Payload B: {id: "story-1", offset: 0, delete: 1, insert: "é"}. The output is éé🌍 (8 UTF-8 bytes); the old run starts inside the first multibyte character and ends inside the last one. Canonical rejection: ✅️validation/🦀️.rs:120 in both cases.

Fix: define language-neutral style inheritance/splice rules and emit adjusted byte runs alongside content. Preserve enough run data for semantic inverse, since a reverse text splice alone cannot restore a removed formatting span. Test deletion, insertion before runs, replacement inside runs and non-ASCII text with the independent oracle.

## P1: Page Creation and Deletion Break Spread Membership

Create source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-page/🦀️.rs:50-57. Payload: {page: copy(base.pages[0]) with id="page-2", frames=[], every layers[].objectIds=[], index:null}. The new page has an existing spreadId=spread-1 but the emitted diff only inserts pages; spread-1.pageIds stays [page-1]. Canonical rejection: ✅️validation/🦀️.rs:141. Missing spread targets are also not guarded.

Delete source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-page/🦀️.rs:47-51. Payload: {id:"page-1"}. It removes the page but keeps spread-1.pageIds=[page-1]. Canonical rejection: ✅️validation/🦀️.rs:136. Any cross-page threads pointing into a deleted page are likewise unaffected by this diff.

Fix: create/remove page and its spread membership atomically, with explicit insertion position retained for inverse. Validate supplied page graph rows before creation. Clean or reject inbound thread references during deletion.

## P1: Frame Creation Admits Broken References and Inherited Collisions

Source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-frame/🦀️.rs:52-70. Checks only page existence, duplicate ids on that page and insertion index. Deriving layer membership is insufficient to validate frame content.

Payload A: {pageId:"page-1", frame: copy(base.pages[0].frames[1]) with id="new", storyId="missing", index:null, layerId:null}. Output includes a text frame naming no story. Canonical rejection: ✅️validation/🦀️.rs:41.

Payload B: {pageId:"page-1", frame: copy(base.parentPages[0].frames[0]) with layerId="layer-1", index:null, layerId:null}. Local id parent-frame collides with the inherited frame on the page. Canonical rejection: ✅️validation/🦀️.rs:154. The same local-only duplicate check also cannot enforce global local-frame uniqueness across pages (line 109).

Fix: validate the candidate frame against global frame ids, parent-local collisions, owning layer, referenced story/link, geometry, columns/insets and candidate thread graph before emitting its insertion.

## P2: Link Creation Allows an Empty Collection Id

Source: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️create-link/🦀️.rs:50-57. Payload: {link:copy(base.links[0]) with id="", index:null}. No existing empty id means the duplicate guard passes and insertion is emitted. Canonical rejection: ✅️validation/🦀️.rs:105 through unique_ids. This is a smaller general id-admission gap; no invalid image-metadata claim is made because the canonical validator does not currently validate link numeric metadata.

Fix: enforce non-empty ids at creation boundaries consistently with unique_ids.

## Scope and Validation Limits

CreateSpread and CreateParagraphStyle already check the relevant scalar/id restrictions in their examined guards. UpdateTextFrame already checks story existence, target kind/existence, finite insets and nonnegative inset sizes. No findings are reported against those existing checks. Character-style nonfinite colors require a native NaN witness and are omitted from established portable findings. This audit executed only the independent JavaScript oracle on projected outputs; production source was unchanged.
