#!/usr/bin/env bun
/**
 * 🏗️ Records the `🧑️contributors` fixtures from this repository's own data.
 *
 * `🧑️‍💻️contributor-documents.json` holds the committed `🧑️‍💻️contributor.json` of every dev
 * verbatim, and `🏁️checkpoint-log.json` holds real `git log` output in the exact pretty format the
 * checkpoint reader asks for. Neither is an answer key: both are artifacts this repository already
 * produced, so a parser that agrees with itself still has to agree with them.
 */
import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const devsDir = join(repoRoot, ".🧬semio", "🦑️repo", "🧑️‍💻️devs");
const fixtures = join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧑️contributors", "🧫️fixtures");

const documents = readdirSync(devsDir)
  .sort()
  .filter((entry) => statSync(join(devsDir, entry)).isDirectory())
  .map((entry) => ({ directory: entry, json: readFileSync(join(devsDir, entry, "🧑️‍💻️contributor.json"), "utf8") }));

const git = (args: string[]): string => execFileSync("git", args, { cwd: repoRoot, encoding: "utf8", maxBuffer: 32 * 1024 * 1024 });
const log = git(["log", "--pretty=format:%H|%aN|%ad|%s", "--date=iso-strict", "-n", "12"]).trimEnd();
const identities = git(["log", "--pretty=format:%ad %aN <%aE>", "--date=format:%Y", "-n", "40"])
  .split("\n")
  .filter((line, index, all) => all.indexOf(line) === index)
  .concat(git(["shortlog", "-sne", "HEAD"]).split("\n").filter((line) => line.trim() !== ""));

writeFileSync(
  join(fixtures, "🧑️‍💻️contributor-documents.json"),
  `${JSON.stringify(
    {
      schema: "semio.repo.contributors.contributor-documents/1",
      $comment: "🧑️ Real 🧑️‍💻️contributor.json documents copied verbatim out of .🧬semio/🦑️repo/🧑️‍💻️devs by 🏗️build-contributors-fixtures.ts.",
      documents,
    },
    null,
    2,
  )}\n`,
);

writeFileSync(
  join(fixtures, "🏁️checkpoint-log.json"),
  `${JSON.stringify(
    {
      schema: "semio.repo.contributors.checkpoint-log/1",
      $comment:
        "🏁️ Real `git log --pretty=format:%H|%aN|%ad|%s --date=iso-strict` output of this repository, plus the identity lines `git log`/`git shortlog` produce for the same commits, recorded by 🏗️build-contributors-fixtures.ts. `malformed` states the lines a reader has to reject rather than guess at.",
      log,
      identities,
      malformed: ["", "no separators at all", "sha|author", "sha|author|date", "|||", "1999 no angle brackets", "Ueli Saluz <ueli@semio-tech.com>", "12345 Too Many Digits <a@b.c>"],
    },
    null,
    2,
  )}\n`,
);

console.log(`[fixtures] ${documents.length} contributor documents, ${log.split("\n").length} checkpoints, ${identities.length} identity lines`);
