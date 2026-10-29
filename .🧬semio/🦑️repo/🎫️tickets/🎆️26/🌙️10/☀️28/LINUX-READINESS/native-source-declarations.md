# Native Source Resource Declarations

Both Go project targets now retain one executable TypeScript runtimeSourceEntries entry. Their three JSON data dependencies are declared through metadata.semio.closure.resources and included explicitly in inputs. The invented runtimeSourceResourceEntries field was removed.

The actual draft-07 project schema passed Ajv validation for both manifests. Both real admission readers produced one executable and three data resources; every resource appeared in inputs. Terminal proof: generated/native-source-project-proof.log.

The search oracle resources are the shards fixture, search schema, and snapshot fixture. The source-policy generator resources are the filesystem descriptor, retirement policy, and canonical caching policy.
