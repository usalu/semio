/** 🧼️ Deploy-readiness proof that the proctor image builds from what a push would contain: every file git would commit
 * (tracked files as they are in the working tree, plus untracked files that are not ignored) copied into a fresh
 * directory — no git-ignored file, no generated source, no host state — and built there with the same Dockerfile and
 * context rules the workflow uses. `.🧬semio` (tickets and host state) is left out: it is no build input, and its paths
 * are too long for a Windows temp directory. `bun deploy_readiness_clean_tree_build.ts`; the image is tagged
 * `deploy-readiness-clean:probe` and removed again, the directory is deleted. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const repo = join(import.meta.dir, "../../../../../../..");
const run = (command: string, args: string[], cwd: string): { status: number; stdout: string } => {
  const done = spawnSync(command, args, { cwd, encoding: "utf8", maxBuffer: 1 << 30 });
  return { status: done.status ?? -1, stdout: `${done.stdout}${done.stderr}` };
};
const listed = run("git", ["-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], repo).stdout.split("\0").filter((path) => path !== "" && !path.startsWith(".🧬semio/"));
const tree = mkdtempSync(join(tmpdir(), "architecture-quiz-clean-"));
try {
  let copied = 0;
  for (const path of listed) {
    const source = join(repo, path);
    if (!existsSync(source) || !statSync(source).isFile()) continue;
    mkdirSync(dirname(join(tree, path)), { recursive: true });
    copyFileSync(source, join(tree, path));
    copied += 1;
  }
  const generated = listed.filter((path) => path.includes("/🤖️generated/") && path.endsWith(".rs")).length;
  console.log(`[DEBUG] ${copied} of ${listed.length} committable files copied to ${tree}; ${generated} of them are committed generated Rust sources`);
  const built = run("docker", ["build", "--progress=plain", "--file", "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile", "--build-arg", "VERSION=0.0.0", "--build-arg", "REVISION=clean-tree", "--tag", "deploy-readiness-clean:probe", "."], tree);
  for (const line of built.stdout.split("\n")) if (/transferring context.*done|Compiling teaching-proctor|catalog architecture|quiz [a-z]+ \(|ERROR|error\[|error:|naming to/u.test(line)) console.log(`[DEBUG] ${line.slice(0, 220)}`);
  console.log(`[DEBUG] docker build exited ${built.status}`);
  if (built.status === 0) {
    console.log(`[DEBUG] built ${run("docker", ["image", "inspect", "--format", "{{.Id}} {{.Size}} bytes", "deploy-readiness-clean:probe"], tree).stdout.trim()}`);
    run("docker", ["image", "rm", "deploy-readiness-clean:probe"], tree);
  }
  process.exitCode = built.status;
} finally {
  rmSync(tree, { recursive: true, force: true });
}
