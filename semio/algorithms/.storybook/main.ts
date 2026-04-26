// This file has been automatically migrated to valid ESM format by Storybook.
// #region 🧲Header
// 💻 semio/algorithms/.storybook/main.ts
// Specs: Keep Storybook wiring aligned with .elements/ui.
// Summary: Configures Storybook for the algorithms bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { StorybookConfig } from "@storybook/react-vite";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { Buffer } from "node:buffer";
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
const semioRsWasmPath = resolve(__dirname, "../../rs/pkg");
const semioUiEntryPath = resolve(__dirname, "../../ui/index.tsx");
const semioJsEntryPath = resolve(__dirname, "../../js/index.ts");

function createNativeAlgorithmsProxyPlugin(options: { readonly repoRootPath: string }) {
  const { repoRootPath } = options;
  function normalizeCsharpJsonKeys(value: any): any {
    if (Array.isArray(value)) return value.map(normalizeCsharpJsonKeys);
    if (!value || typeof value !== "object") return value;
    const out: any = {};
    for (const [k, v] of Object.entries(value)) {
      const nk = k.length > 0 ? k[0].toLowerCase() + k.slice(1) : k;
      out[nk] = normalizeCsharpJsonKeys(v);
    }
    return out;
  }
  return {
    name: "semio:algorithms-native-algorithms-proxy",
    apply: "serve" as const,
    configureServer(server: any) {
      server.middlewares.use("/api/native-algorithms/execute", async (req: any, res: any, next: any) => {
        try {
          if ((req.method || "").toUpperCase() !== "POST") return next();

          const chunks: Buffer[] = [];
          for await (const c of req) chunks.push(Buffer.isBuffer(c) ? c : Buffer.from(c));
          const rawBody = Buffer.concat(chunks).toString("utf-8");
          const body = JSON.parse(rawBody || "{}") as {
            language?: "python" | "go" | "rust" | "csharp";
            operation?: "flatten" | "delete";
            kit?: unknown;
            design?: unknown;
            designId?: string;
            pieceIds?: unknown;
            connectionIds?: unknown;
          };

          const language = body.language ?? "python";
          const operation = body.operation ?? "flatten";

          const bridgePayload = {
            op: operation,
            kit: body.kit ?? {},
            design: body.design ?? {},
            designId: body.designId ?? "",
            pieceIds: Array.isArray(body.pieceIds) ? body.pieceIds : [],
            connectionIds: Array.isArray(body.connectionIds) ? body.connectionIds : [],
          };

          let result: unknown;
          if (operation === "flatten") {
            const { nativeFlattenDesign } = await import("../nativeAlgorithmAdapter");
            result = await nativeFlattenDesign(bridgePayload.kit as any, bridgePayload.designId, "ts");
          } else if (language === "python") {
            const py = spawnSync(
              "uv",
              [
                "run",
                "python",
                "-c",
                [
                  "import json,sys",
                  "import main",
                  "body=json.load(sys.stdin)",
                  "op=body.get('op')",
                  "kit=body.get('kit') or {}",
                  "design=body.get('design') or {}",
                  "dg=body.get('designId') or ''",
                  "pg=body.get('pieceIds') or []",
                  "cg=body.get('connectionIds') or []",
                  "if op=='delete':",
                  "    out=main.deletePiecesAndConnectionsInDesignDict(kit,design,pg,cg)",
                  "else:",
                  "    raise Exception('unknown op: '+str(op))",
                  "print(json.dumps({'ok': True, 'result': out}))",
                ].join("\n"),
              ],
              {
                cwd: resolve(repoRootPath, "semio/py"),
                input: JSON.stringify(bridgePayload),
                encoding: "utf-8",
              },
            );
            if (py.status !== 0) throw new Error((py.stderr || py.stdout || "python native bridge failed").trim());
            const out = JSON.parse(py.stdout || "{}") as { ok?: boolean; result?: unknown; error?: string };
            if (!out.ok) throw new Error(out.error || "python native bridge error");
            result = out.result;
          } else if (language === "go") {
            const go = spawnSync("go", ["run", "-mod=mod", "."], {
              cwd: resolve(repoRootPath, "semio/algorithms/native-bridges/go"),
              input: JSON.stringify(bridgePayload),
              env: { ...process.env, GOWORK: "off" },
              encoding: "utf-8",
            });
            if (go.status !== 0) throw new Error((go.stderr || "go native bridge failed").trim());
            const out = JSON.parse(go.stdout || "{}") as { ok?: boolean; result?: unknown; error?: string };
            if (!out.ok) throw new Error(out.error || "go native bridge error");
            result = out.result;
          } else if (language === "rust") {
            const rs = spawnSync("cargo", ["run", "-q"], {
              cwd: resolve(repoRootPath, "semio/algorithms/native-bridges/rs"),
              input: JSON.stringify({ ...bridgePayload, design: operation === "delete" ? bridgePayload.design : undefined }),
              encoding: "utf-8",
            });
            if (rs.status !== 0) throw new Error((rs.stderr || "rust native bridge failed").trim());
            const out = JSON.parse(rs.stdout || "{}") as { ok?: boolean; result?: unknown; error?: string };
            if (!out.ok) throw new Error(out.error || "rust native bridge error");
            result = out.result;
          } else if (language === "csharp") {
            const cs = spawnSync("dotnet", ["run", "--project", "./csharp-native-bridge.csproj", "-q"], {
              cwd: resolve(repoRootPath, "semio/algorithms/native-bridges/csharp"),
              input: JSON.stringify(bridgePayload),
              encoding: "utf-8",
            });
            if (cs.status !== 0) throw new Error((cs.stderr || "csharp native bridge failed").trim());
            const out = JSON.parse(
              String(cs.stdout || "")
                .split("\n")
                .reverse()
                .find((l) => l.trim().startsWith("{")) || "{}",
            ) as { ok?: boolean; result?: unknown; error?: string };
            if (!out.ok) throw new Error(out.error || "csharp native bridge error");
            result = normalizeCsharpJsonKeys(out.result);
          } else {
            throw new Error(`unsupported language: ${language}`);
          }

          res.statusCode = 200;
          res.setHeader("Content-Type", "application/json");
          res.end(JSON.stringify({ result }));
        } catch (e: any) {
          res.statusCode = 500;
          res.setHeader("Content-Type", "application/json");
          res.end(JSON.stringify({ error: String(e?.message ?? e) }));
        }
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
      "@semio/rs-wasm": semioRsWasmPath,
      "@semio/assets": resolve(__dirname, "../../assets"),
      "@semio/algorithms": algorithmsEntryPath,
    };
    config.assetsInclude = [...(config.assetsInclude ?? []), "**/*.wasm"];
    config.server = config.server || {};
    config.server.proxy = config.server.proxy || {};
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

    config.plugins.push(createNativeAlgorithmsProxyPlugin({ repoRootPath }));

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

    // ⚙️Storybook builds use code-splitting; module-worker (semio/js spawns `./worker.ts` with `{type:"module"}`) requires ES worker format (iife rejects split chunks).
    config.worker = {
      ...(config.worker || {}),
      format: "es",
    };

    return config;
  },
};

export default config;
