# Process Workshop Quota Ownership Diagnosis

## Observed Failure

The actual peer log `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️process-fresh-surfaces-close-2.log` opens the Process editor and publishes main, document and catalogue patches. Workshop then fails with `SurfaceReconcileUsage { nodes:47, items:1600, bytes:8391431 }` against `{ max_nodes:128, max_items:4097, max_bytes:8388608 }`. Bytes exceed the quota by exactly2823; node/item limits were not exceeded. This is the census's first rejected projection, not a completed workshop total or a measurement of all live heap bytes.

No quota, UI contract, runtime, or Process source was changed for this read-only diagnosis. No Process/native-runtime/Wasm compile was launched. Current source may differ from the peer binary; exact ABI sizes and the complete workshop ownership total still require the targeted native/fresh-component measurements below.

## Confirmed Ownership Mismatch

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`:

- `SurfaceSemanticCensusCursor::step` at1040 charges `3 * size_of::<TreeNode>()`. `TreeNode` is the UI contract's `BuiltNode`, which contains key/component/accessibility/menu inline.
- The subsequent key, label, icon, accessibility, action scope/name/capability visits charge `UiText::capacity()` again through `owner`, multiplied by3. `UiText` is an inline `[u8;512]` plus length (`🦀️action.rs:22`), not a heap String. These bytes already belong to the containing node/component/binding allocation. Counting their capacity again is not a second physical owner.
- Node bindings at1080 and row-actions at966 charge `capacity * size_of::<ActionBinding/RowAction>()`, multiplied by3, then charge their inline text fields separately as well. A real allocated backing is being charged, but its inline fields are additionally charged as if they were detached allocations.
- The projected node page at1250/1280 adds `size_of::<FlatPresentedNode>()` on top of semantic usage. That struct itself embeds a TreeNode. Thus the baseline is `size_of::<FlatPresentedNode>() + 3*size_of::<TreeNode>()` before all repeated inline-text charges. This is a conservative proxy, not an exact physical allocation inventory.
- The literal three-copy multiplier is not joined to actual owner identity. `build_record_owned` at3258 moves the presented node's fields into the retained record, while the fresh Upsert path clones one other record. UiValue list/map `credited_clone` retains arena aliases rather than duplicating all descendant storage. Existing retained roots, new records, patch records, traversal backing and alias metadata require separately named ownership credits; a universal multiply-by3 cannot establish their exact overlap.

An exact small counterexample follows directly from these paths: one node with one `ActionBinding { args:None, capability:None }` and the same node with two such bindings both use the already-allocated32-slot binding backing. The second insertion allocates no extra backing and both action texts are inline in those slots. Nevertheless the census adds another `2 * 512 * 3 = 3072` bytes for its scope/name. Similarly, filling a previously absent inline TreeItem icon changes no allocation size but adds1536 census bytes. These are source-derived accounting deltas, not yet executed native measurements.

## Real Physical Cost Must Remain Accounted

The Process workshop producer `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛠️workshop/🦀️component.rs:34` creates one row-action list for each installed machine (remove action), and one node binding for each uninstalled catalogue machine (add action). `installed_catalogs` at editor/🦀️component.rs:1616 materializes the five built-in catalogues plus contributed catalogues, then the workshop renders every machine in each.

`UiFixedList::try_reserve` (`🦀️action.rs:166`) actually reservesN elements, and bindings useN=32 (`🦀️document.rs:87`). Therefore one binding really does acquire `32 * size_of::<ActionBinding>()` backing, and one RowAction acquires `32 * size_of::<RowAction>()`. Changing census capacity to initialized length would undercount real allocation. Each ActionBinding contains two inline UiText fields in ActionId, an optional UiValue whose largest variant carries UiText, and optional UiText capability; RowAction additionally embeds icon and optional label. Their exact native/Wasm sizes must be obtained from the compiler, not guessed from serialized JSON lengths.

The recent fixed-list Vec repair avoids initialized empty payloads but still reservesN; it does not remove this physical backing cost. The fixed reconciliation vectors also reserve their complete capacities up front (`SurfaceFixedVec::default` at40), which a per-used-node proxy is not an exact account of either. Correcting double-counting alone is not evidence that the complete workshop fits8MiB, because this log stopped partway through its traversal.

## Small Native TDD Plan

1. Add a strict language-neutral ownership fixture under the runtime reconcile domain: TreeItem icon None/Some, one/two bindings without args, one/two row-actions, shared nested UiValue aliases, and first/last backing allocation. The expected logical ownership relationships are architecture-neutral; native and Wasm report their own `size_of` layout facts. Use serde_json plus the existing Node/Ajv oracle to validate identical fixture output.
2. In the existing small UI-runtime test target, drive the actual semantic census for those minimal roots. Record each admitted backing's pointer, capacity and element size, plus inline-field location within its parent. Assert unchanged physical backing for the one-to-two binding case and the icon toggle. Current code should RED with an unexplained3072/1536 increment respectively. Preserve semantic item/work counters separately from resident-memory credits.
3. Add fixed per-category diagnostic counters: node/record shells, binding backing, row-action backing, child backing, unique arena owners, aliases, and inline bytes already included. No allocation-address map or whole-tree recount in an interactive turn; the domain cursor must carry exact ownership categories and preadmitted owner identities incrementally.
4. Repair only the runtime census after those RED tests, retaining8MiB/128nodes/4097items and fixed production grants. Count each physical allocation once per actual owner, inline fields through their enclosing storage, and each shared alias shell separately from its unique payload. Do not replace physical capacity with semantic length, use a blanket smaller multiplier, or skip large bindings.
5. Then run a small actual workshop producer fixture with the exact active catalogues, and the fresh component surface gate. If genuinely allocated backing still exceeds8MiB, redesign retained binding/row-action storage admission (logical maximum separate from exact admitted capacity) with its owner; do not raise the quota or silently remove workshop machines.

The measured runtime fact is the2823-byte quota crossing. The repeated-inline accounting defect and real32-slot costs are source-confirmed. A full physical-byte decomposition and proof that the real workshop fits remain pending, explicitly separate from this read-only diagnosis.
