import Ajv from "ajv";
import { readFileSync } from "node:fs";
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true });
const shared = "✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️preview-revisions/";
const fixture = read(shared + "🔣️.json");
if (!ajv.validate(read(shared + "🧬️schema.json"), fixture)) throw Error(JSON.stringify(ajv.errors));
for (const row of fixture.cases) {
  const visible = row.currentRevision !== null && row.sourceRevision.length === 32 && Buffer.from(row.sourceRevision).equals(Buffer.from(row.currentRevision));
  if (visible !== row.visible) throw Error(row.id);
}
for (const [artifact, fill] of [["◻️2d", "🔣️fill"], ["🧊️3d", "📑️fill"]]) {
  const base = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/" + artifact + "/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/";
  const tool = base + "🎭️modes/✏️edit/🛠️tools/" + fill + "/";
  if (!ajv.validate(read(tool + "🧬️schema/🔣️.json"), read(tool + "🧪️tests/🧫️fixtures/🎞️partial-tick.json"))) throw Error(JSON.stringify(ajv.errors));
  const transient = base + "🫧️transient/";
  const vector = transient + "🧪️tests/🧫️fixtures/🏁️set-solve/✅️set-solve-applied/";
  const before = read(vector + "📸️snapshot/⬅️before/🔣️.json"), after = read(vector + "📸️snapshot/➡️after/🔣️.json");
  const mutation = read(vector + "🦠️mutation/🔣️.json").SetSolve, diff = read(vector + "🔺️diff/🔣️.json");
  if (!ajv.validate(read(transient + "🧬️schema/🧬️mutations/🏁️set-solve/🧬️schema/🔣️.json"), mutation)) throw Error(JSON.stringify(ajv.errors));
  if (JSON.stringify({...before, ...mutation}) !== JSON.stringify(after) || JSON.stringify({...before, ...diff}) !== JSON.stringify(after)) throw Error("whole transient differs");
}
const envelope = read("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️graph-fill-envelope/🔣️.json");
for (const [count, admitted] of [[envelope.acceptedAssignments, true], [envelope.oversizedAssignments, false]]) {
  const assignments = Object.fromEntries(Array.from({length: count as number}, (_, i) => ["slot-" + i, "t".repeat(envelope.idBytes)]));
  const payload = {sourceRevision: Array(32).fill(7), assignments, contradiction: false, done: true, trace: []};
  const bytes = Buffer.byteLength(JSON.stringify(payload));
  if ((bytes <= envelope.tickMaximumBytes) !== admitted) throw Error("tick bound differs");
  if (admitted && (Buffer.byteLength(JSON.stringify({CommitFill: {payloadJson: JSON.stringify(payload)}})) <= 65536)) throw Error("accepted case misses original command defect");
}
console.log("validated 6 revision cases, 2 graph tick/mutation carriers and 2 envelope cases with Ajv");

const viewer = read("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️viewer-preview/🔣️.json");
if (!ajv.validate(read("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️viewer-preview/🧬️schema.json"), viewer)) throw Error(JSON.stringify(ajv.errors));
const captions = { en: ["No current result. Run Fill in the editor.", "Showing authored pins. No current solve is available."], de: ["Kein aktuelles Ergebnis. Füllen im Editor starten.", "Angelegte Fixierungen werden angezeigt. Kein aktuelles Ergebnis verfügbar."] };
for (const row of viewer.cases) if (captions[row.locale as "en" | "de"][Number(row.authored)] !== row.message) throw Error(row.id);
console.log("[DEBUG] Ajv and independent viewer language/authored projection oracle passed " + viewer.cases.length + " cases");
