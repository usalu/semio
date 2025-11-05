import mdx from "@mdx-js/rollup";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig(async () => {
  // normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite");
  return {
    resolve: {
      alias: {
        "@semio/js": path.resolve(__dirname, "../js"),
      },
    },
    plugins: [
      tailwind.default(),
      {
        ...mdx({
          remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
          rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
          providerImportSource: "@mdx-js/react",
        }),
        enforce: 'pre',
      },
      react(),
      wasm(),
      topLevelAwait(), // needed for older browsers to run wasm
    ],
    optimizeDeps: {
      include: ["golden-layout"],
      exclude: ["@semio/js"],
      esbuildOptions: {
        target: "es2020",
      },
    },
    ssr: {
      noExternal: ["golden-layout"],
    },
  };
});
