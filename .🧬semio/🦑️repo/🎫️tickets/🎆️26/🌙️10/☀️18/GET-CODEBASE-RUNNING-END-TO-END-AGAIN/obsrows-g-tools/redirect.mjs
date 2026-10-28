/** 🪞 obsrows-g: node --import hook that serves the staged mirror of a live library module in place of the live file for every importer (MIRROR_FROM / MIRROR_TO are absolute paths). */
import {registerHooks} from "node:module";
import {pathToFileURL} from "node:url";
const from=pathToFileURL(process.env.MIRROR_FROM).href,to=pathToFileURL(process.env.MIRROR_TO).href;
registerHooks({resolve(specifier,context,next){const resolved=next(specifier,context);return resolved.url===from?{...resolved,url:to}:resolved;}});
