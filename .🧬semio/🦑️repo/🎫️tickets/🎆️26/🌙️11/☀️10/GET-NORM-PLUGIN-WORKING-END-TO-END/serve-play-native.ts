#!/usr/bin/env repo
import {resolve,join} from "node:path";
import {createServer} from "vite";
const root = resolve(import.meta.dirname, "../../../../../../..");
const play = join(root, "🏢️semio-tech/🎡️play");
const port = Number(process.env.SEMIO_TECH_PLAY_PORT ?? "6034");
Object.assign(process.env, {
  SEMIO_TECH_PLAY_SCOPE:"standards", SEMIO_TECH_PLAY_FROZEN:"true", SEMIO_BUILD_MODE:"prototype",
  SEMIO_VITE_CACHE_DIRECTORY:join(root, ".cache/b/p/m/vite", "play-standards-native-" + port)
});
const server = await createServer({
  mode:"prototype", root:play, configFile:join(play,"🏗️builder/🌐️vite/🟦️.ts"),
  configLoader:"native", dev:{preTransformRequests:false},
  server:{host:"127.0.0.1",port,strictPort:true,preTransformRequests:false},
});
process.once("SIGTERM",async()=>{await server.close();process.exit(0);});
process.once("SIGINT",async()=>{await server.close();process.exit(0);});
await server.listen();
server.printUrls();
