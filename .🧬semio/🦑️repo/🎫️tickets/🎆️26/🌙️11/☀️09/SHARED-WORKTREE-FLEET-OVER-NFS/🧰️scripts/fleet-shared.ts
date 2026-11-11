#!/usr/bin/env bun
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [,, worktree, boundary] = process.argv;
const {compileIgnoreV1, scanSharedV1} = await import(boundary!);
const ignore = compileIgnoreV1(readFileSync(join(worktree!, ".gitignore"), "utf8"));
const paths: string[] = [];
await scanSharedV1(worktree!, ignore, undefined, (entry: {path: string}) => { paths.push(entry.path); });
process.stdout.write(paths.join("\n") + "\n");
