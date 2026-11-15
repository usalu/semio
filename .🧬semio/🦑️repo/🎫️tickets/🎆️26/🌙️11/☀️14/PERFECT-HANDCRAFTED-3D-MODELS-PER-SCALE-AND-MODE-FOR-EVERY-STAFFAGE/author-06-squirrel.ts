import { run } from "./author-06-run.ts";
import { notesFor } from "./author-06-notes.ts";
await run("red-squirrel", { emit: { "breath-nose": "nose" }, notes: notesFor("red-squirrel"), struct: () => ({ spine: ["spine-r", "spine-f"] }) });
