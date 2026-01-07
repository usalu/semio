# Plan: Update Schema Location Reference

## Task
Update gqlgen.yml to reference the new schema location at `graphql/repo/schema.graphql` instead of the old location.

## Analysis
- The GraphQL schema has been moved from `go/repo/schema.graphql` to `graphql/repo/schema.graphql`
- The gqlgen.yml file needs to be updated to point to the new relative path
- The schema file header also needs updating to reflect the new location

## Steps
1. Update `go/repo/gqlgen.yml` schema path from `schema.graphql` to `../../graphql/repo/schema.graphql`
2. Update the header comment in `graphql/repo/schema.graphql` to reflect the correct path
3. Create checkpoint with modified files
