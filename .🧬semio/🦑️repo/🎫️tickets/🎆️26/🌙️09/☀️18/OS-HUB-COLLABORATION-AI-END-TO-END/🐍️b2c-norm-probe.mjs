/** 📕️ Generic norm interaction probe — one shim for all fifteen standards (slice B2c).
 *
 * B2b's `🐍️b2b-norm-probe.mjs` staged a hand-written din4108 document read off a committed fixture,
 * which does not generalise: the fifteen norm snapshots have fifteen different shapes (flat scalars
 * plus an ordered layer list in din4108, a composed `s.stdio.semio` child in en1990, nested
 * `BTreeMap` catalogues in vdi3805/iso16757). This shim instead reads the document the app currently
 * has open out of its own Inputs window — which renders exactly `pack::json` of the live snapshot
 * (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:182`) — moves ONE top-level numeric scalar, and stages
 * that back through `setSnapshot`. The payload therefore cannot drift from the codec under test, and
 * the same code exercises every standard.
 *
 * Usage: node 🐍️b2c-norm-probe.mjs <variant> <port>
 */
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

const [variant, port] = process.argv.slice(2);
if (!variant || !port) throw new Error("usage: b2c-norm-probe.mjs <variant> <port>");

/** 🧾️ The live document, read out of the Inputs window's rendered JSON. */
const readDocument = async (page) => {
  const raw = await page.evaluate((id) => {
    const window = document.getElementById(id);
    return window === null ? null : (window.innerText ?? "");
  }, `norm-${variant}-inputs`);
  if (raw === null) throw new Error(`inputs window norm-${variant}-inputs is absent`);
  // 🧩️ `render_text_chunks` pages the pretty JSON across several UiText carriers and `innerText`
  // re-joins them with a newline, which lands inside a quoted string whenever a page boundary does.
  // Every newline here is either that artefact or the printer's own separator, and neither is
  // load-bearing in JSON, so all of them go.
  const text = raw.replace(/[\n\r\t]/g, "");
  const start = text.indexOf("{");
  const end = text.lastIndexOf("}");
  if (start < 0 || end <= start) throw new Error(`inputs window carries no JSON document: ${text.slice(0, 200)}`);
  return JSON.parse(text.slice(start, end + 1));
};

/** 🔢️ Moves exactly one top-level scalar, preferring a float, then an integer, then a boolean: a
 * one-field payload is the narrowest thing `XMutation::from_snapshot` can decompose, so the witness
 * is the publication path and not the breadth of the diff. */
const moveOneField = (document) => {
  const entries = Object.entries(document);
  const float = entries.find(([, value]) => typeof value === "number" && !Number.isInteger(value));
  const integer = entries.find(([, value]) => typeof value === "number");
  const flag = entries.find(([, value]) => typeof value === "boolean");
  const picked = float ?? integer ?? flag;
  if (picked === undefined) throw new Error(`no movable top-level scalar in ${Object.keys(document).join(",")}`);
  const [key, value] = picked;
  const next = typeof value === "boolean" ? !value : Number.isInteger(value) ? value + 1 : Number((value + 0.5).toFixed(4));
  return { key, from: value, to: next, snapshot: JSON.stringify({ ...document, [key]: next }) };
};

await runInteractionProbe({
  plugin: `norm-${variant}`,
  variant,
  port: Number(port),
  action: "setSnapshot",
  args: {
    snapshot: async (page) => {
      const staged = moveOneField(await readDocument(page));
      console.log(`staged ${variant} ${staged.key} ${staged.from} -> ${staged.to}`);
      return staged.snapshot;
    },
  },
});
