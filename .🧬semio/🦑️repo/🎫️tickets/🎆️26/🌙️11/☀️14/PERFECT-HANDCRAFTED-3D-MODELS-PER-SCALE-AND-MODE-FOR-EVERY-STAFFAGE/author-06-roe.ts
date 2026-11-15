import { run } from "./author-06-run.ts";
import { notesFor } from "./author-06-notes.ts";
await run("roe-deer", { emit: { "breath-nose": "nose-skin" }, fixY: { "plume-back": 0.8 }, notes: notesFor("roe-deer"), auto: () => ({ over: { "ear-n": { fw: 0.6, zhK: 0.9 }, "ear-f": { fw: 0.6, zhK: 0.9 }, ear: { zhK: 0.9 } } }), struct: () => ({ spine: ["pelvis", "lumbar", "thorax"], skipJoints: ["ear-n", "ear-f"] }) });
