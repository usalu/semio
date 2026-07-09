# Ticket

## Todos

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

## Changes

## Log

# Log

## Task

Add Toml, Yaml, Sql, and Graphql language plugins to the repo CLI language registry.

## Implementation

1. Added TomlLanguage struct (lines 1424-1467):
   - Extensions: `.toml`
   - Sections parsed via `[section]` and `[[array]]` headers
   - Supports comments with `#`
   - Custom ParseSections method for TOML-style section headers

2. Added YamlLanguage struct (lines 1473-1491):
   - Extensions: `.yaml`, `.yml`
   - Supports comments with `#`
   - Uses indent scoping (YAML's natural structure)
   - No sections or definitions (data format)

3. Added SqlLanguage struct (lines 1497-1519):
   - Extensions: `.sql`
   - Regions: `-- #region` / `-- #endregion`
   - Definition detection for CREATE TABLE, VIEW, PROCEDURE, FUNCTION, TRIGGER, INDEX, TYPE, SCHEMA, DATABASE, SEQUENCE, MATERIALIZED VIEW
   - Supports OR REPLACE and IF NOT EXISTS modifiers
   - Comments with `--`

4. Added GraphqlLanguage struct (lines 1525-1547):
   - Extensions: `.graphql`, `.gql`
   - Regions: `# #region` / `# #endregion`
   - Definition detection for type, interface, enum, input, union, scalar, query, mutation, subscription, fragment, and extend variants
   - Comments with `#`

5. Registered all four languages in languageRegistry (lines 1560-1563)

## Summary

Bulk close
