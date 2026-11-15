import { run } from "./author-06-run.ts";
import { notesFor } from "./author-06-notes.ts";
await run("red-fox", { emit: { "breath-nose": "face", "breath-mouth": "face" }, notes: notesFor("red-fox"), struct: () => ({ spine: ["pelvis", "lumbar", "thorax"], skipJoints: ["ear-n", "ear-f"] }) });
