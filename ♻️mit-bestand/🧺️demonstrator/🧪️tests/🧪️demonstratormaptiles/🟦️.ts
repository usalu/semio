import Ajv from "ajv";
import {geoMercator} from "d3-geo";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import cancellationSchema from "./🧬️schema/🔣️.json";
import cancellationFixture from "./🧫️cases.json";

type TestSource = { readonly directory: string; readonly url: string };


function mapTileFunctionV1(path:string,name:string,bindings:Record<string,unknown>):unknown{
 const source=readFileSync(new URL(path,import.meta.url),"utf8"),file=ts.createSourceFile("tiles.ts",source,ts.ScriptTarget.Latest,true),node=file.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("Owned map tile function required");const emitted=ts.transpileModule(node.getText(file).replace(/^export\s+/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;return Function(...Object.keys(bindings),emitted+";return "+name)(...Object.values(bindings));
}

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: { demonstratorGisMapTileServeMode: (command: "build" | "serve") => string; DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER: number; DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR: number },
): Promise<void> {
  const { demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR } = dependencies;
  const { describe, expect, it } = vitest;

  describe("demonstratorGisMapTileServeMode", () => {
    it("checks only prefetched canaries against the actual GIS plan and independent Mercator projection",async()=>{
      expect(new Ajv({strict:true}).compile(cancellationSchema)(cancellationFixture)).toBe(true);
      const planSchema=JSON.parse(readFileSync(new URL("../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🧬️schema/🔣️.json",import.meta.url),"utf8")),bounds=planSchema.properties.bounds.default,coordinate=mapTileFunctionV1("../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts","lonLatToTileXY",{planSchema})as(lon:number,lat:number,z:number)=>{x:number;y:number},range=mapTileFunctionV1("../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts","tileRangeForBounds",{lonLatToTileXY:coordinate}),plan=mapTileFunctionV1("../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts","listMapTilesForBounds",{planSchema,tileRangeForBounds:range})as(bounds:unknown,zMin:number,zMax:number)=>{z:number;x:number;y:number}[],actual=plan(bounds,0,DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR),projection=geoMercator().scale(1/(2*Math.PI)).translate([0.5,0.5]),independent:{z:number;x:number;y:number}[]=[];
      for(let z=0;z<=DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR;z++){const n=2**z,sw=projection([bounds.west,bounds.south])!,ne=projection([bounds.east,bounds.north])!,tile=(point:number[])=>({x:Math.max(0,Math.min(n-1,Math.floor(point[0]!*n))),y:Math.max(0,Math.min(n-1,Math.floor(point[1]!*n)))}),a=tile(sw),b=tile(ne);for(let x=Math.min(a.x,b.x);x<=Math.max(a.x,b.x);x++)for(let y=Math.min(a.y,b.y);y<=Math.max(a.y,b.y);y++)independent.push({z,x,y});}
      expect(actual).toEqual(independent);for(const row of cancellationFixture.canaries){const matches=(value:{z:number;x:number;y:number})=>value.z===row.coordinate.z&&value.x===row.coordinate.x&&value.y===row.coordinate.y;expect(actual.some(matches)).toBe(row.included);expect(independent.some(matches)).toBe(row.included);}
      for(const row of cancellationFixture.cacheRuns){const roots={osm:"owned-osm",vt:"owned-vt"},key=(kind:"osm"|"vt",tile:{z:number;x:number;y:number})=>resolve(roots[kind],tile.z+"/"+tile.x+"/"+tile.y+(kind==="vt"?".pbf":".png")),all=[...plan(bounds,0,DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER).map(tile=>key("osm",tile)),...actual.map(tile=>key("vt",tile))],cached=new Set(row.warm?all:[]),checked:string[]=[],controller=new AbortController(),prefetch=mapTileFunctionV1("../../🔨️modules/📦️site/🗺️map-tiles/🟦️.ts","prefetchDemonstratorMapTiles",{GIS_MAP_DEFAULT_PREFETCH_BOUNDS:bounds,DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER,DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR,resolve,mapTileCacheRoots:()=>roots,existsSync:(path:string)=>{checked.push(path);return cached.has(path);},prefetchMapTiles:async(options:{signal:AbortSignal;skipExisting:boolean;bounds:unknown;zMinRaster:number;zMaxRaster:number;zMinVector:number;zMaxVector:number})=>{expect(options.signal).toBe(controller.signal);expect(options).toMatchObject({skipExisting:true,bounds,zMinRaster:0,zMaxRaster:DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER,zMinVector:0,zMaxVector:DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR});const skipped=all.filter(path=>cached.has(path)).length;for(const path of all)cached.add(path);return{downloaded:all.length-skipped,skipped,failed:0,cancelled:false};}})as(root:string,signal:AbortSignal)=>Promise<void>;
        await prefetch("workspace",controller.signal);expect(checked,row.name).toEqual([key("vt",cancellationFixture.canaries.find(value=>value.kind==="vt"&&value.included)!.coordinate),key("osm",cancellationFixture.canaries.find(value=>value.kind==="osm"&&value.included)!.coordinate)]);
      }
    });

    it("propagates the issued cancellation lifetime through prefetch and publication",async()=>{
      expect(new Ajv({strict:true}).compile(cancellationSchema)(cancellationFixture)).toBe(true);
      const buildSource=readFileSync(new URL("../../🔨️modules/📦️site/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("site.ts",buildSource,ts.ScriptTarget.Latest,true),forwarded:string[]=[];const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="prefetchDemonstratorMapTiles")forwarded.push(node.arguments[1]?.getText(tree)??"");ts.forEachChild(node,visit);};visit(tree);expect(forwarded).toEqual(["context.invocation.control.signal"]);
      for(const row of cancellationFixture.cases){const controller=new AbortController(),reason=Error("Owned tile prefetch cancellation"),observed={prefetchCalls:0,signalForwarded:false,buildCalls:0,cancelled:false,failure:"none"};if(row.abortBefore)controller.abort(reason);
        const prefetch=mapTileFunctionV1("../../🔨️modules/📦️site/🗺️map-tiles/🟦️.ts","prefetchDemonstratorMapTiles",{GIS_MAP_DEFAULT_PREFETCH_BOUNDS:{},DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER,DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR,resolve,existsSync:()=>true,mapTileCacheRoots:()=>({osm:"osm",vt:"vt"}),prefetchMapTiles:async(options:{signal?:AbortSignal})=>{observed.prefetchCalls++;observed.signalForwarded=options.signal===controller.signal;if(row.abortDuring)controller.abort(reason);return{downloaded:1,skipped:0,failed:row.failed,cancelled:controller.signal.aborted};}})as(root:string,signal:AbortSignal)=>Promise<void>;
        const build=mapTileFunctionV1("../../🔨️modules/📦️site/🟦️.ts","runBuildScript",{resolve,prefetchDemonstratorMapTiles:prefetch,scriptCompilerAdmissionControlV1:()=>({}),readBuildMode:()=>"release",frameworkOsViteArtifactInputsV1:()=>[],repositoryCachePartitionRootV1:()=>"products",join:(...parts:string[])=>parts.join("/"),buildViteArtifact:async(options:{signal:AbortSignal})=>{expect(options.signal).toBe(controller.signal);observed.buildCalls++;}})as(context:unknown,args:string[])=>Promise<void>;
        try{await build({root:"site",repoRoot:"workspace",invocation:{control:{signal:controller.signal}}},[]);}catch(error){observed.cancelled=error===reason;observed.failure=observed.cancelled?"cancelled":error instanceof Error&&error.message.includes("tile prefetch failed")?"tiles":"unexpected";}expect(observed,row.name).toEqual(row.expected);
      }
    });

    it("bundles map tiles for production Vite builds", () => {
      expect(demonstratorGisMapTileServeMode("build")).toBe("bundle");
    });

    it("caps static ship prefetch below full offline-play depth", () => {
      expect(DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER).toBeLessThanOrEqual(10);
      expect(DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR).toBeLessThanOrEqual(10);
    });

    it("keeps dev serve on fetch unless GIS_MAP_TILE_SERVE_MODE overrides", () => {
      const previous = process.env.GIS_MAP_TILE_SERVE_MODE;
      delete process.env.GIS_MAP_TILE_SERVE_MODE;
      expect(demonstratorGisMapTileServeMode("serve")).toBe("fetch");
      process.env.GIS_MAP_TILE_SERVE_MODE = "bundle";
      expect(demonstratorGisMapTileServeMode("serve")).toBe("bundle");
      if (previous === undefined) delete process.env.GIS_MAP_TILE_SERVE_MODE;
      else process.env.GIS_MAP_TILE_SERVE_MODE = previous;
    });
  });
}
