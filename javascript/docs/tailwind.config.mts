import type { Config } from "tailwindcss";
// import sharedConfig from "@semio/core/tailwind.config";

const config: Pick<Config, "content" | "presets"> = {
  content: [
    "./src/pages/**/*.{tsx,mdx}",
    "../core/components/**/**.{ts,tsx,mdx}"
  ],
  // presets: [sharedConfig],
};

export default config;