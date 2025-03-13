import type { Config } from "tailwindcss";
// import sharedConfig from "@semio/core/tailwind.config";

const config: Pick<Config, "content" | "presets"> = {
  content: [
    "./src/pages/**/*.tsx",
    "../core/components/**/**.{js,ts,jsx,tsx,mdx}"
  ],
  // presets: [sharedConfig],
};

export default config;