# Plan

1. Add TomlLanguage struct with:
   - Extensions: `.toml`
   - No regions (TOML has no standard region convention)
   - No definitions (TOML is data, not code)
   - Sections via top-level `[section]` and `[[array]]` headers

2. Add YamlLanguage struct with:
   - Extensions: `.yaml`, `.yml`
   - No regions
   - No definitions (YAML is data, not code)
   - Sections via top-level keys or `# region` comments if desired

3. Add SqlLanguage struct with:
   - Extensions: `.sql`
   - Regions using `-- #region` / `-- #endregion`
   - Definition detection for `CREATE TABLE`, `CREATE VIEW`, `CREATE PROCEDURE`, `CREATE FUNCTION`, etc.

4. Add GraphqlLanguage struct with:
   - Extensions: `.graphql`, `.gql`
   - Regions using `# #region` / `# #endregion`
   - Definition detection for `type`, `interface`, `enum`, `input`, `union`, `scalar`, `query`, `mutation`, `subscription`, `fragment`

5. Register all four languages in `languageRegistry`
