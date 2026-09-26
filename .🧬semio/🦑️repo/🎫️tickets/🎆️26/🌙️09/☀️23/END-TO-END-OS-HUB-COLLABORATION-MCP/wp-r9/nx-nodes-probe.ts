#!/usr/bin/env bun
/** ⏱️ R9 item 1 probe: how long nx's own node inference (plugins, no dependency scan) takes, and what it yields. */
const started = performance.now();
const internals = await import("/Users/ueli/Documents/semio/node_modules/nx/dist/src/devkit-internals.js");
const { readNxJson } = await import("/Users/ueli/Documents/semio/node_modules/nx/dist/src/config/nx-json.js");
const root = "/Users/ueli/Documents/semio";
const result = await internals.retrieveProjectConfigurationsWithAngularProjects(root, readNxJson(root));
const projects = Object.values(result.projects) as { name: string; targets?: Record<string, unknown> }[];
console.log(`projects=${projects.length} targets=${projects.reduce((sum, project) => sum + Object.keys(project.targets ?? {}).length, 0)} ms=${Math.round(performance.now() - started)}`);
process.exit(0);
