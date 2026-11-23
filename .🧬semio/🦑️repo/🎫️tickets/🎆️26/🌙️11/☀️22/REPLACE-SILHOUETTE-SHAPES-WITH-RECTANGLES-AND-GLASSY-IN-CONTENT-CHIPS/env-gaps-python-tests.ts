/** 🐍 Runs the migrated teaching Python test modules through the course route: pinned native libs, then uv at the course environment. usage: repo env-gaps-python-tests.ts <module...>; output in 🗑️generated */
import { spawn } from "node:child_process";
import { join } from "node:path";
import { mkdirSync } from "node:fs";
import { provisionNative } from "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/🧱️native/🟦️.ts";

const workspace = "/Users/ueli/Documents/semio";
const project = join(workspace, "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python");
const cache = join(workspace, ".🧬semio/🎓️teaching/⚡️cache");
mkdirSync(join(cache, "tests"), { recursive: true });
const native = await provisionNative({ onProgress: ({ step, fraction }) => { if (fraction === undefined || fraction === 1) console.error(`[native] ${step}`); } });
const environment = join(cache, "python");
const env = { ...process.env, ...native?.environment, UV_PYTHON_INSTALL_DIR: join(workspace, ".cache/b/t/m/python/interpreters"), UV_PYTHON_DOWNLOADS: "never", PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1", SEMIO_TEST_MANIM: "1", SEMIO_TEACHING_LANGUAGE: process.env.SEMIO_TEACHING_LANGUAGE ?? "en", SEMIO_TEACHING_PROFILE: "final", UV_PROJECT_ENVIRONMENT: environment, SEMIO_TOOL_PYTHON: join(environment, "bin/python"), UV_BUILD_CONSTRAINT: join(project, "build-constraints.txt"), SEMIO_TEACHING_CACHE_DIR: join(cache, "fonts"), SEMIO_TEACHING_OUTPUT_ROOT: join(cache, "tests"), PLAYWRIGHT_BROWSERS_PATH: join(workspace, ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright") };
const child = spawn("repo", ["tools", "exec", "uv", "--", "run", "--project", project, "--python", "3.12", "--group", "test", "python", ...(process.argv[2] === "--script" ? process.argv.slice(3) : ["-m", "unittest", "-v", ...process.argv.slice(2)])], { cwd: workspace, env, stdio: "inherit" });
child.on("exit", code => { process.exitCode = code ?? 1; });
