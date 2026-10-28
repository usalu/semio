/** 🧪️ Original export frame preparation agrees with independent stable sorting and RFC6902 overrides. */
import {test,expect} from "bun:test";
import {run,repositoryRoot} from "./🔮️oracle.mjs";
test("bounded frame paint addresses and shared metadata collisions",()=>{const result=run(repositoryRoot());expect(result.frames.runs).toBe(45);expect(result.frames.rows).toBe(1746);expect(result.frames.cancelled).toBe(28);expect(result.metadata.queries).toBe(27);});
