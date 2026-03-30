// #region 🔖Header
// 💻 semio/algorithms/.storybook/main.ts
// Specs: Keep Storybook wiring aligned with .elements/ui.
// Summary: Configures Storybook for the algorithms bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { StorybookConfig } from "@storybook/react-vite";
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { createRequire } from "node:module";
import net from "node:net";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "path";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRootPath = resolve(__dirname, "../../..");
const elementsUiDir = resolve(__dirname, "../../../elements/ui");
const elementsUiEntryPath = resolve(elementsUiDir, "index.tsx");
const algorithmsEntryPath = resolve(__dirname, "../index.ts");
const semioUiEntryPath = resolve(__dirname, "../../ui/index.tsx");
const semioJsEntryPath = resolve(__dirname, "../../js/index.ts");

function isPortOpen(host: string, port: number, timeoutMs: number): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = new net.Socket();
    const done = (value: boolean) => {
      try {
        socket.destroy();
      } catch {}
      resolve(value);
    };
    socket.setTimeout(timeoutMs);
    socket.once("connect", () => done(true));
    socket.once("timeout", () => done(false));
    socket.once("error", () => done(false));
    socket.connect(port, host);
  });
}

function createEnsureNativeSemioProxyDevServerPlugin(options: { readonly host: string; readonly port: number; readonly repoRootPath: string }) {
  const { host, port, repoRootPath } = options;

  let proc: ChildProcessWithoutNullStreams | undefined;
  let starting: Promise<void> | undefined;

  async function ensureStarted(): Promise<void> {
    if (await isPortOpen(host, port, 250)) return;
    if (proc) return;
    if (starting) return starting;

    starting = (async () => {
      if (await isPortOpen(host, port, 250)) return;

      // Storybook should be able to execute native algorithms without requiring the semio engine.
      // We scope this to algorithms Storybook so it does not affect the original implementations.
      // eslint-disable-next-line no-console
      console.log(`[DEBUG] semio/algorithms: starting semio native proxy dev server on ${host}:${port}`);

      proc = spawn("uv", ["run", "python", "-c", "import main; main.start_native_algorithms_rest()"], {
        cwd: resolve(repoRootPath, "semio/py"),
        env: {
          ...process.env,
          HOST: "0.0.0.0",
          PORT: String(port),
        },
        stdio: "pipe",
      });

      proc.stdout.on("data", (buf) => {
        // eslint-disable-next-line no-console
        console.log(String(buf).trimEnd());
      });
      proc.stderr.on("data", (buf) => {
        // eslint-disable-next-line no-console
        console.error(String(buf).trimEnd());
      });
      proc.on("exit", (code, signal) => {
        // eslint-disable-next-line no-console
        console.log(`[DEBUG] semio/algorithms: native proxy dev server exited (code=${code}, signal=${signal})`);
        proc = undefined;
        starting = undefined;
      });

      // Wait for port to become available (best-effort).
      const deadlineMs = Date.now() + 20_000;
      while (Date.now() < deadlineMs) {
        if (await isPortOpen(host, port, 250)) return;
        await new Promise((r) => setTimeout(r, 250));
      }
    })();

    try {
      await starting;
    } finally {
      starting = undefined;
    }
  }

  return {
    name: "semio:ensure-native-semio-proxy-dev-server",
    apply: "serve",
    async configureServer(server: any) {
      await ensureStarted();
      server.httpServer?.once?.("close", () => {
        if (!proc) return;
        // eslint-disable-next-line no-console
        console.log("[DEBUG] semio/algorithms: stopping native proxy dev server");
        try {
          proc.kill("SIGTERM");
        } catch {}
        proc = undefined;
      });
    },
  };
}

function getAbsolutePath(value: string): string {
  try {
    return dirname(require.resolve(join(value, "package.json")));
  } catch {
    return dirname(require.resolve(join("../../../elements/ui/node_modules", value, "package.json")));
  }
}

const config: StorybookConfig = {
  stories: ["./stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"],
  addons: [getAbsolutePath("@storybook/addon-vitest"), getAbsolutePath("@storybook/addon-docs")],
  framework: {
    name: getAbsolutePath("@storybook/react-vite"),
    options: {},
  },
  docs: {},
  typescript: {
    reactDocgen: "react-docgen-typescript",
  },
  core: {
    disableTelemetry: true,
  },
  async viteFinal(config) {
    config.resolve = config.resolve || {};
    config.resolve.alias = {
      ...(config.resolve.alias || {}),
      "@elements/ui/elements": elementsUiEntryPath,
      "@elements/ui": elementsUiDir,
      "@semio/ui": semioUiEntryPath,
      "@semio/js": semioJsEntryPath,
      "@semio/assets": resolve(__dirname, "../../assets"),
      "@semio/algorithms": algorithmsEntryPath,
    };
    config.server = config.server || {};
    config.server.proxy = {
      ...(config.server.proxy || {}),
      "/api": {
        target: "http://127.0.0.1:2507",
        changeOrigin: true,
      },
    };
    config.server.fs = {
      ...(config.server.fs || {}),
      allow: Array.from(new Set([...(config.server.fs?.allow || []), repoRootPath])),
    };

    config.plugins = config.plugins || [];
    const indicesToRemove: number[] = [];
    for (let i = 0; i < config.plugins.length; i++) {
      const plugin: any = config.plugins[i];
      if (plugin === "@mdx-js/rollup" || (plugin && typeof plugin === "object" && plugin.name === "@mdx-js/rollup")) {
        indicesToRemove.push(i);
        continue;
      }
      if (plugin instanceof Promise) {
        try {
          const resolved: any = await plugin;
          if (resolved && typeof resolved === "object" && resolved.name === "storybook:mdx-plugin") {
            indicesToRemove.push(i);
          }
        } catch (e) {}
      }
    }
    for (let i = indicesToRemove.length - 1; i >= 0; i--) {
      config.plugins.splice(indicesToRemove[i], 1);
    }

    const mdx = await import("@mdx-js/rollup");
    config.plugins.push(
      mdx.default({
        remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
        rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
      }),
    );

    config.plugins.push(
      createEnsureNativeSemioProxyDevServerPlugin({
        host: "127.0.0.1",
        port: 2507,
        repoRootPath,
      }),
    );

    config.optimizeDeps = config.optimizeDeps || {};
    config.optimizeDeps.include = [...(config.optimizeDeps.include || []), "golden-layout"];
    config.optimizeDeps.exclude = Array.from(new Set([...(config.optimizeDeps.exclude || []), "@semio/ui", "@semio/js", "@semio/assets", "@elements/ui", "@elements/ui/elements"]));
    config.optimizeDeps.esbuildOptions = {
      ...config.optimizeDeps.esbuildOptions,
      target: "es2020",
    };

    config.mode = "development";
    config.define = {
      ...config.define,
      "process.env.NODE_ENV": JSON.stringify("development"),
    };

    return config;
  },
};

export default config;
