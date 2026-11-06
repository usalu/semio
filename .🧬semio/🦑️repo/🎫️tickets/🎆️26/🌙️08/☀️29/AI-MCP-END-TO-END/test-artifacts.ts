import{mkdirSync}from"node:fs";import{join}from"node:path";const root=join(import.meta.dirname,"🗑️generated","t");mkdirSync(root,{recursive:true});process.env.SEMIO_TEST_ARTIFACT_DIR=root;
