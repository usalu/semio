import { basename } from "node:path";
import { answer } from "./library.ts";
export const result = { answer, name: basename("/fixture/input") };
