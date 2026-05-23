import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🔎Resolves monorepo root (directory containing root package.json named `semio`). */
export function getWorkspaceRoot(): string {
  const fromEnv = process.env.REPO_ROOT?.trim();
  if (fromEnv) return resolve(fromEnv);
  let dir = process.cwd();
  for (let i = 0; i < 30; i++) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      try {
        const j = JSON.parse(readFileSync(pkg, "utf8")) as { name?: string };
        if (j.name === "semio") return dir;
      } catch {
        /* ignore */
      }
    }
    const up = dirname(dir);
    if (up === dir) break;
    dir = up;
  }
  return process.cwd();
}

function defaultCliBin(root: string): string {
  const win = process.platform === "win32";
  return join(root, "repo", "client", win ? "client.exe" : "client");
}

export function resolveCliBin(root = getWorkspaceRoot()): string {
  const fromEnv = process.env.REPO_CLI_BIN?.trim();
  if (fromEnv) return resolve(fromEnv);
  return defaultCliBin(root);
}

/** 📡Runs repo client with `--json` and returns parsed GraphQL payload (`data` object). */
export function runCliGraphql(
  query: string,
  variables: Record<string, unknown> = {},
  options?: { cwd?: string; repoRoot?: string },
): unknown {
  const root = options?.repoRoot ?? getWorkspaceRoot();
  const cwd = options?.cwd ?? root;
  const bin = resolveCliBin(root);
  const vars = JSON.stringify(variables ?? {});
  const args = ["--repo", root, "--json", "graphql", "--query", query, "-v", vars];
  let stdout: string;
  try {
    stdout = execFileSync(bin, args, {
      cwd,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    });
  } catch (e: unknown) {
    const err = e as { stderr?: Buffer; status?: number; message?: string };
    const msg = err.stderr?.toString?.() ?? err.message ?? String(e);
    throw new Error(`[repo/cli] exit ${err.status ?? "?"}: ${msg}`);
  }
  const lines = stdout
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  let last: unknown;
  for (const line of lines) {
    try {
      last = JSON.parse(line) as { data?: unknown; errors?: { message: string }[] };
    } catch {
      continue;
    }
  }
  if (!last || typeof last !== "object") {
    throw new Error(`[repo/cli] no JSON lines in stdout: ${stdout.slice(0, 500)}`);
  }
  const payload = last as { data?: unknown; errors?: { message: string }[] };
  if (payload.errors?.length) {
    throw new Error(`[repo/cli] graphql errors: ${payload.errors.map((x) => x.message).join("; ")}`);
  }
  return payload.data ?? payload;
}

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 🧭Package dir for `@repo/lib` (…/repo/lib/js). */
export function getLibRoot(): string {
  return resolve(__dirname, "..");
}
