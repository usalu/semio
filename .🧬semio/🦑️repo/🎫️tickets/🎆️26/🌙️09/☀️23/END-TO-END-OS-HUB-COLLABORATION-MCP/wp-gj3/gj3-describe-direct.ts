#!/usr/bin/env bun
import { describePluginComponent } from "./links/fresh-component.ts";
const rc = describePluginComponent('/Users/ueli/Documents/semio', "semio-s-plugin-wfc", '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc');
console.error("describe rc", rc);
process.exit(rc);
