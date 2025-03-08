/** @type {import("eslint").Linter.Config} */
module.exports = {
    extends: ["@semio/core/eslint.config.mjs"],
    ignorePatterns: ["gatsby-types.d.ts"],
};