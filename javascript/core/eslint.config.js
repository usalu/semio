import js from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
import turboPlugin from "eslint-plugin-turbo";
import tseslint from "typescript-eslint";
import pluginReactHooks from "eslint-plugin-react-hooks";
import pluginReact from "eslint-plugin-react";
import globals from "globals";

/** @type {import("eslint").Linter.Config} */
const eslintConfig = [
    js.configs.recommended,
    eslintConfigPrettier,
    ...tseslint.configs.recommended,
    pluginReact.configs.flat.recommended,
    {
        plugins: {
            turbo: turboPlugin,
            "react-hooks": pluginReactHooks,
        },
        // settings: { react: { version: "detect" } },
        rules: {
            ...pluginReactHooks.configs.recommended.rules,
            // React scope no longer necessary with new JSX transform.
            "react/react-in-jsx-scope": "off",
        },
        languageOptions: {
            ...pluginReact.configs.flat.recommended.languageOptions,
            globals: {
                ...globals.serviceworker,
                ...globals.browser,
            },
        },
        ignores: ["dist/**"],
    },
];

export default eslintConfig;