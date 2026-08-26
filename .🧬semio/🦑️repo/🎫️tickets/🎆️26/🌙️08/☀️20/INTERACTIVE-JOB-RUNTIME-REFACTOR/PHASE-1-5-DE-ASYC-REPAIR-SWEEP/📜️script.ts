const root = decodeURIComponent(new URL("../../../../../../../../", import.meta.url).pathname);
if (Bun.argv[2] === "lowpoly-runtime-awaits") {
  type Edit = { start: number; end: number; before: string; after: string; kind: string };
  const lowpolyRoot = `${root}✏️s/🔌️plugins/💠️lowpoly`;
  const journal: { files: { source: string; beforeHash: string; afterHash: string; edits: Edit[] }[] } = { files: [] };
  const sources = new Map<string, string>();
  for await (const relative of new Bun.Glob("**/*.rs").scan({ cwd: lowpolyRoot, onlyFiles: true })) sources.set(relative, await Bun.file(`${lowpolyRoot}/${relative}`).text());
  const main = "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs";
  const mainHash = new Bun.CryptoHasher("sha256").update(sources.get(main)!).digest("hex");
  if (mainHash !== "6285e24738a9e62302ee5e6f6663e31c589767b95b580a63ed696fe0d7bf74bf") throw new Error(`Lowpoly helper source guard drift: ${mainHash}`);
  const edits = new Map<string, Edit[]>();
  const addEdit = (relative: string, edit: Edit) => {
    const target = edits.get(relative) ?? [];
    target.push(edit);
    edits.set(relative, target);
  };
  const replaceExact = (relative: string, before: string, after: string, expected: number, kind: string) => {
    const source = sources.get(relative)!;
    const matches = [...source.matchAll(new RegExp(before.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "g"))];
    if (matches.length !== expected) throw new Error(`${kind} census drift: ${matches.length} != ${expected}`);
    for (const match of matches) addEdit(relative, { start: match.index, end: match.index + before.length, before, after, kind });
  };
  const callEnd = (source: string, open: number) => {
    let depth = 0;
    let quote = "";
    let escaped = false;
    for (let index = open; index < source.length; index += 1) {
      const char = source[index];
      if (quote) {
        if (escaped) escaped = false;
        else if (char === "\\") escaped = true;
        else if (char === quote) quote = "";
        continue;
      }
      if (char === '"' || char === "'") {
        quote = char;
        continue;
      }
      if (char === "(") depth += 1;
      else if (char === ")" && --depth === 0) return index + 1;
    }
    throw new Error(`unterminated call at ${open}`);
  };
  const awaitCalls = (relative: string, needle: string, expected: number, kind: string, startAt = 0) => {
    const source = sources.get(relative)!;
    let cursor = startAt;
    let found = 0;
    while ((cursor = source.indexOf(needle, cursor)) >= 0) {
      const open = cursor + needle.length - 1;
      let end: number;
      try {
        end = callEnd(source, open);
      } catch (error) {
        throw new Error(`${kind} call guard failed: ${relative}:${open}: ${error}`);
      }
      if (source.slice(end, end + 6) !== ".await") addEdit(relative, { start: end, end, before: "", after: ".await", kind });
      found += 1;
      cursor = end;
    }
    if (found !== expected) throw new Error(`${kind} census drift: ${relative}:${found} != ${expected}`);
  };
  replaceExact(main, "pub fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult", "pub async fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult", 1, "testkit-dispatch-async");
  replaceExact(main, "pub fn render(app: &mut LowpolyApp, body_key: &str) -> String", "pub async fn render(app: &mut LowpolyApp, body_key: &str) -> String", 1, "testkit-render-async");
  replaceExact(main, "pub fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32)", "pub async fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32)", 1, "testkit-select-face-async");
  let dispatchTyped = 0;
  let handleAction = 0;
  for (const [relative, source] of sources) {
    const typed = source.split(".dispatch_typed(").length - 1;
    const actions = source.split(".handle_action(").length - 1;
    if (typed) awaitCalls(relative, ".dispatch_typed(", typed, "dispatch-typed-await");
    if (actions) awaitCalls(relative, ".handle_action(", actions, "handle-action-await");
    dispatchTyped += typed;
    handleAction += actions;
  }
  if (dispatchTyped !== 12 || handleAction !== 6) throw new Error(`Lowpoly runtime call census drift: ${JSON.stringify({ dispatchTyped, handleAction })}`);
  awaitCalls(main, "app.render(", 1, "plugin-render-await");
  let helperDispatch = 0;
  let helperRender = 0;
  let helperSelect = 0;
  for (const [relative, source] of sources) {
    const use = source.match(/use crate::editor::lowpoly::testkit::\{([^}]*)\};/);
    if (!use) continue;
    const names = use[1].split(",").map((name) => name.trim());
    for (const [name, kind] of [["dispatch", "testkit-dispatch-await"], ["render", "testkit-render-await"], ["select_face", "testkit-select-face-await"]] as const) {
      if (!names.includes(name)) continue;
      const testSource = source.slice(use.index! + use[0].length);
      const count = [...testSource.matchAll(new RegExp(`\\b${name}\\(`, "g"))].length;
      awaitCalls(relative, `${name}(`, count, kind, use.index! + use[0].length);
      if (name === "dispatch") helperDispatch += count;
      else if (name === "render") helperRender += count;
      else helperSelect += count;
    }
  }
  if (helperDispatch !== 19 || helperRender !== 4 || helperSelect !== 4) throw new Error(`Lowpoly helper call census drift: ${JSON.stringify({ helperDispatch, helperRender, helperSelect })}`);
  awaitCalls(main, "crate::editor::lowpoly::testkit::dispatch(", 1, "qualified-testkit-dispatch-await");
  replaceExact(main, "render(&mut a, \"lowpoly.play.nope\")", "render(&mut a, \"lowpoly.play.nope\").await", 1, "local-testkit-render-await");
  awaitCalls(main, "testkit::assert_two_instances_converge::<EditorApp<LowpolyPlayApp>, _>(", 1, "testkit-convergence-await");
  awaitCalls(main, "testkit::assert_ingest_idempotent::<EditorApp<LowpolyPlayApp>, _>(", 1, "testkit-idempotence-await");
  for (const [relative, fileEdits] of edits) {
    const source = sources.get(relative)!;
    if (new Set(fileEdits.map((edit) => `${edit.start}:${edit.end}`)).size !== fileEdits.length) throw new Error(`overlapping Lowpoly runtime edits: ${relative}`);
    let output = source;
    for (const edit of fileEdits.toSorted((left, right) => right.start - left.start)) {
      if (output.slice(edit.start, edit.end) !== edit.before) throw new Error(`Lowpoly runtime span guard failed: ${relative}:${edit.start}`);
      output = output.slice(0, edit.start) + edit.after + output.slice(edit.end);
    }
    await Bun.write(`${lowpolyRoot}/${relative}`, output);
    journal.files.push({ source: `✏️s/🔌️plugins/💠️lowpoly/${relative}`, beforeHash: new Bun.CryptoHasher("sha256").update(source).digest("hex"), afterHash: new Bun.CryptoHasher("sha256").update(output).digest("hex"), edits: fileEdits.toSorted((left, right) => left.start - right.start) });
  }
  await Bun.write(new URL("./lowpoly-runtime-awaits-span-journal.json", import.meta.url), `${JSON.stringify(journal, null, 2)}\n`);
  console.log(JSON.stringify({ files: journal.files.length, dispatchTyped, handleAction, helperDispatch, helperRender, helperSelect, edits: journal.files.reduce((sum, file) => sum + file.edits.length, 0) }));
  process.exit(0);
}
if (Bun.argv[2] === "shared-pure-roots") {
  type Edit = { start: number; end: number; before: string; after: string; kind: string };
  const paths = {
    vcs: "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs",
    store: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs",
    plugin: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs",
    builder: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs",
  } as const;
  const expectedHashes: Record<string, string> = {
    [paths.vcs]: "91eaec59f176b7e962181411c51eddf1a99de021c91a4d1919cc7e4a507b0531",
    [paths.plugin]: "5291875caf1471110688f93021ad4091811a34cc978aef6e7e4d0d59fccc83cb",
  };
  const journal: { files: { source: string; beforeHash: string; afterHash: string; edits: Edit[] }[] } = { files: [] };
  const sources = new Map<string, string>();
  for (const relative of Object.values(paths)) {
    const source = await Bun.file(`${root}${relative}`).text();
    const hash = new Bun.CryptoHasher("sha256").update(source).digest("hex");
    if (expectedHashes[relative] && hash !== expectedHashes[relative]) throw new Error(`shared pure-root source guard drift: ${relative}:${hash}`);
    sources.set(relative, source);
  }
  const edits = new Map<string, Edit[]>();
  const replaceExact = (relative: string, before: string, after: string, expected: number, kind: string) => {
    const source = sources.get(relative)!;
    const matches = [...source.matchAll(new RegExp(before.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "g"))];
    if (matches.length !== expected) throw new Error(`${kind} census drift: ${relative}:${matches.length} != ${expected}`);
    const target = edits.get(relative) ?? [];
    for (const match of matches) target.push({ start: match.index, end: match.index + before.length, before, after, kind });
    edits.set(relative, target);
  };
  replaceExact(paths.vcs, "pub async fn apply_mutation<", "pub fn apply_mutation<", 1, "vcs-apply-mutation-sync");
  replaceExact(paths.plugin, "pub async fn new(snapshot: &'a P, history: &'a HistoryView) -> Self", "pub fn new(snapshot: &'a P, history: &'a HistoryView) -> Self", 1, "artifact-view-new-sync");
  replaceExact(paths.plugin, "pub async fn empty() -> Self", "pub fn empty() -> Self", 1, "history-view-empty-sync");
  replaceExact(paths.plugin, "pub async fn from_definition(definition: &AppDefinition) -> Self", "pub fn from_definition(definition: &AppDefinition) -> Self", 1, "action-registry-from-definition-sync");
  replaceExact(paths.plugin, "pub async fn snapshot(&self) -> Result<A::Snapshot, Fault>", "pub fn snapshot(&self) -> Result<A::Snapshot, Fault>", 1, "vcs-app-snapshot-sync");
  replaceExact(paths.plugin, "pub async fn backbone_ref(&self) -> Option<&store::ArtifactBackboneRef>", "pub fn backbone_ref(&self) -> Option<&store::ArtifactBackboneRef>", 1, "vcs-app-backbone-ref-sync");
  replaceExact(paths.store, "apply_mutation(&snapshot, operation).await", "apply_mutation(&snapshot, operation)", 5, "store-apply-mutation-await");
  replaceExact(paths.store, "apply_mutation(&folded, operation).await", "apply_mutation(&folded, operation)", 1, "store-apply-mutation-fold-await");
  replaceExact(paths.store, "apply_mutation(&running, &mutation).await", "apply_mutation(&running, &mutation)", 2, "store-apply-mutation-running-await");
  replaceExact(paths.store, "apply_mutation(pre, &operation).await", "apply_mutation(pre, &operation)", 1, "store-apply-mutation-pre-await");
  replaceExact(paths.store, "apply_mutation(&restored, back_operation).await", "apply_mutation(&restored, back_operation)", 1, "store-apply-mutation-restore-await");
  replaceExact(paths.plugin, "HistoryView::empty().await", "HistoryView::empty()", 4, "history-view-empty-await");
  replaceExact(paths.plugin, "ArtifactView::new(&snapshot, &history).await", "ArtifactView::new(&snapshot, &history)", 3, "artifact-view-new-await");
  replaceExact(paths.plugin, ".snapshot().await", ".snapshot()", 9, "vcs-app-snapshot-await");
  replaceExact(paths.plugin, ".backbone_ref().await", ".backbone_ref()", 3, "vcs-app-backbone-ref-await");
  replaceExact(paths.plugin, "AppActionRegistry::from_definition(&definition).await", "AppActionRegistry::from_definition(&definition)", 1, "registry-definition-await");
  replaceExact(paths.plugin, "resolve_ready(AppActionRegistry::from_definition(def))", "AppActionRegistry::from_definition(def)", 2, "registry-definition-ready");
  replaceExact(paths.plugin, "AppActionRegistry::from_definition(&app.definition).await", "AppActionRegistry::from_definition(&app.definition)", 3, "registry-definition-test-await");
  replaceExact(paths.builder, "resolve_ready(crate::app::AppActionRegistry::from_definition(def))", "crate::app::AppActionRegistry::from_definition(def)", 4, "registry-definition-builder-ready");
  for (const [relative, fileEdits] of edits) {
    const source = sources.get(relative)!;
    if (new Set(fileEdits.map((edit) => edit.start)).size !== fileEdits.length) throw new Error(`overlapping shared pure-root edits: ${relative}`);
    let output = source;
    for (const edit of fileEdits.toSorted((left, right) => right.start - left.start)) {
      if (output.slice(edit.start, edit.end) !== edit.before) throw new Error(`shared pure-root span guard failed: ${relative}:${edit.start}`);
      output = output.slice(0, edit.start) + edit.after + output.slice(edit.end);
    }
    await Bun.write(`${root}${relative}`, output);
    journal.files.push({ source: relative, beforeHash: new Bun.CryptoHasher("sha256").update(source).digest("hex"), afterHash: new Bun.CryptoHasher("sha256").update(output).digest("hex"), edits: fileEdits.toSorted((left, right) => left.start - right.start) });
  }
  await Bun.write(new URL("./shared-pure-roots-span-journal.json", import.meta.url), `${JSON.stringify(journal, null, 2)}\n`);
  console.log(JSON.stringify({ files: journal.files.length, edits: journal.files.reduce((sum, file) => sum + file.edits.length, 0) }));
  process.exit(0);
}
if (Bun.argv[2] === "lowpoly-residual") {
  const lowpolyRoot = `${root}✏️s/🔌️plugins/💠️lowpoly`;
  const journal: {
    files: { source: string; beforeHash: string; afterHash: string; edits: { start: number; end: number; before: string; after: string; kind: string }[] }[];
  } = { files: [] };
  let decorativeAsync = 0;
  let staleAwaits = 0;
  for await (const relative of new Bun.Glob("**/*.rs").scan({ cwd: lowpolyRoot, onlyFiles: true })) {
    const sourcePath = `${lowpolyRoot}/${relative}`;
    const source = await Bun.file(sourcePath).text();
    const edits: { start: number; end: number; before: string; after: string; kind: string }[] = [];
    for (const match of source.matchAll(/\basync\s+(?=fn\b)/g)) {
      if (/\#\[semio_framework_async_macros::async_test\]\s*$/.test(source.slice(Math.max(0, match.index - 160), match.index))) continue;
      edits.push({ start: match.index, end: match.index + match[0].length, before: match[0], after: "", kind: "decorative-async" });
      decorativeAsync += 1;
    }
    for (const match of source.matchAll(/\.await\b/g)) {
      edits.push({ start: match.index, end: match.index + match[0].length, before: match[0], after: "", kind: "stale-await" });
      staleAwaits += 1;
    }
    if (edits.length === 0) continue;
    let output = source;
    for (const edit of edits.toSorted((left, right) => right.start - left.start)) {
      if (output.slice(edit.start, edit.end) !== edit.before) throw new Error(`Lowpoly residual span guard failed at ${relative}:${edit.start}`);
      output = output.slice(0, edit.start) + edit.after + output.slice(edit.end);
    }
    await Bun.write(sourcePath, output);
    journal.files.push({
      source: `✏️s/🔌️plugins/💠️lowpoly/${relative}`,
      beforeHash: new Bun.CryptoHasher("sha256").update(source).digest("hex"),
      afterHash: new Bun.CryptoHasher("sha256").update(output).digest("hex"),
      edits,
    });
  }
  if (decorativeAsync !== 210 || staleAwaits !== 4) throw new Error(`Lowpoly residual census drift: ${JSON.stringify({ decorativeAsync, staleAwaits })}`);
  await Bun.write(new URL("./lowpoly-deasync-residual-span-journal.json", import.meta.url), `${JSON.stringify(journal, null, 2)}\n`);
  console.log(JSON.stringify({ files: journal.files.length, decorativeAsyncRemoved: decorativeAsync, staleAwaitsRemoved: staleAwaits }));
  process.exit(0);
}
if (Bun.argv[2] === "lowpoly-compiler-spans") {
  const errorLogPath = decodeURIComponent(new URL("./lowpoly-retained-native.txt", import.meta.url).pathname);
  const lowpolyRoot = "✏️s/🔌️plugins/💠️lowpoly/";
  const errorLog = await Bun.file(errorLogPath).text();
  const locations = [...errorLog.matchAll(/error\[E0053\][\s\S]*?\n\s*-->\s+(.+):(\d+):(\d+)/g)]
    .map((match) => ({ path: match[1], line: Number(match[2]), column: Number(match[3]) }))
    .filter((location) => location.path.includes(lowpolyRoot));
  if (locations.length !== 102) throw new Error(`Lowpoly E0053 census drift: ${locations.length}`);
  const journal: {
    compilerLogHash: string;
    files: { source: string; beforeHash: string; afterHash: string; edits: { start: number; end: number; before: string; after: string; line: number; column: number }[] }[];
  } = {
    compilerLogHash: new Bun.CryptoHasher("sha256").update(errorLog).digest("hex"),
    files: [],
  };
  const byPath = new Map<string, typeof locations>();
  for (const location of locations) {
    const relative = location.path.slice(location.path.indexOf(lowpolyRoot));
    const existing = byPath.get(relative) ?? [];
    existing.push({ ...location, path: relative });
    byPath.set(relative, existing);
  }
  for (const [relative, fileLocations] of [...byPath].toSorted(([left], [right]) => left.localeCompare(right))) {
    const sourcePath = decodeURIComponent(new URL(relative, `file://${root}`).pathname);
    const source = await Bun.file(sourcePath).text();
    const lineStarts = [0];
    for (let index = 0; index < source.length; index += 1) if (source[index] === "\n") lineStarts.push(index + 1);
    const edits = fileLocations.map((location) => {
      const lineStart = lineStarts[location.line - 1];
      const prefixStart = Math.max(0, lineStart - 2_048);
      const matches = [...source.slice(prefixStart, lineStart + location.column).matchAll(/\basync\s+(?=fn\b)/g)];
      const match = matches.at(-1);
      if (!match) throw new Error(`compiler span has no preceding async function: ${relative}:${location.line}:${location.column}`);
      const start = prefixStart + match.index;
      return { start, end: start + match[0].length, before: match[0], after: "", line: location.line, column: location.column };
    });
    if (new Set(edits.map((edit) => edit.start)).size !== edits.length) throw new Error(`duplicate compiler spans in ${relative}`);
    let output = source;
    for (const edit of edits.toSorted((left, right) => right.start - left.start)) {
      if (output.slice(edit.start, edit.end) !== edit.before) throw new Error(`Lowpoly span guard failed at ${relative}:${edit.line}`);
      output = output.slice(0, edit.start) + edit.after + output.slice(edit.end);
    }
    await Bun.write(sourcePath, output);
    journal.files.push({
      source: relative,
      beforeHash: new Bun.CryptoHasher("sha256").update(source).digest("hex"),
      afterHash: new Bun.CryptoHasher("sha256").update(output).digest("hex"),
      edits,
    });
  }
  await Bun.write(new URL("./lowpoly-deasync-compiler-span-journal.json", import.meta.url), `${JSON.stringify(journal, null, 2)}\n`);
  console.log(JSON.stringify({ files: journal.files.length, decorativeAsyncRemoved: locations.length }));
  process.exit(0);
}
const sourcePath = `${root}🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs`;
const journalPath = new URL("./mesh-deasync-span-journal.json", import.meta.url).pathname;
const expectedHash = "af83e0e22641aa98aa02b38dc029786341c52c50021c3b90086ada6162b9ae6f";
const marker = "//#region Tests";

const source = await Bun.file(sourcePath).text();
const hash = new Bun.CryptoHasher("sha256").update(source).digest("hex");
if (hash !== expectedHash) throw new Error(`mesh source guard drift: ${hash}`);
const cutoff = source.indexOf(marker);
if (cutoff < 0) throw new Error("mesh test marker absent");

const edits: { start: number; end: number; before: string; after: string; kind: string }[] = [];
for (const match of source.slice(0, cutoff).matchAll(/\basync\s+(?=fn\b)/g)) {
  edits.push({ start: match.index, end: match.index + match[0].length, before: match[0], after: "", kind: "decorative-async" });
}
for (const match of source.matchAll(/\.await\b/g)) {
  edits.push({ start: match.index, end: match.index + match[0].length, before: match[0], after: "", kind: "stale-await" });
}
if (edits.filter((edit) => edit.kind === "decorative-async").length !== 92 || edits.filter((edit) => edit.kind === "stale-await").length !== 627) {
  throw new Error(`mesh span census drift: ${JSON.stringify({ async: edits.filter((edit) => edit.kind === "decorative-async").length, awaits: edits.filter((edit) => edit.kind === "stale-await").length })}`);
}

let output = source;
for (const edit of edits.toSorted((left, right) => right.start - left.start)) {
  if (output.slice(edit.start, edit.end) !== edit.before) throw new Error(`mesh span guard failed at ${edit.start}`);
  output = output.slice(0, edit.start) + edit.after + output.slice(edit.end);
}
await Bun.write(sourcePath, output);
await Bun.write(journalPath, `${JSON.stringify({ source: sourcePath.slice(root.length), beforeHash: hash, afterHash: new Bun.CryptoHasher("sha256").update(output).digest("hex"), edits }, null, 2)}\n`);
console.log(JSON.stringify({ file: sourcePath.slice(root.length), decorativeAsyncRemoved: 92, staleAwaitsRemoved: 627 }));
