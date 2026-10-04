import{readFileSync,writeFileSync,mkdirSync,existsSync}from"node:fs";
import{resolve,dirname,relative}from"node:path";
import{createHash}from"node:crypto";
import{isDeepStrictEqual}from"node:util";

const ticket=resolve(import.meta.dir,"..");let root=ticket;while(!existsSync(resolve(root,"nx.json"))||!existsSync(resolve(root,"Cargo.toml")))root=dirname(root);const command=process.argv[2],epoch=process.argv[3]??"1",sha=(body:string)=>createHash("sha256").update(body).digest("hex");
if(!existsSync(resolve(root,"nx.json"))||!existsSync(resolve(root,"Cargo.toml")))throw Error("Actual Root authority unresolved");
if(command!=="stage"||!/^[1-9]\d*$/.test(epoch))throw Error("Unknown finite Canvas scene command");
const generated=resolve(ticket,"🗑️generated/neutral-render-owner"),output=resolve(generated,"canvas-scene-target-source-ready-"+epoch+".json");mkdirSync(generated,{recursive:true});if(existsSync(output))throw Error("Immutable scene authority exists");
const priorPath=resolve(generated,"canvas-physical-owner-proposal-1.json"),priorSource=readFileSync(priorPath,"utf8"),prior=JSON.parse(priorSource),canonical="🧰️framework/🔨️modules/🖱️ui/🖼️canvas",canvasPath=canonical+"/🦀️.rs",canvasRow=prior.rows.find((row:any)=>row.path===canvasPath),enginePath="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs";
if(!canvasRow?.after)throw Error("Exact authored neutral Canvas predecessor absent");
function pair(path:string,before:string|null,after:string){let start=0,end=0;const a=before??"";while(start<a.length&&start<after.length&&a[start]===after[start])start++;while(end<a.length-start&&end<after.length-start&&a.at(-end-1)===after.at(-end-1))end++;const removed=a.slice(start,a.length-end),inserted=after.slice(start,after.length-end);return{path,before,after,inverseBody:before,beforeHash:before===null?null:sha(before),afterHash:sha(after),forward:{start,removed,inserted},inverse:{start,removed:inserted,inserted:removed},inverseExact:after.slice(0,start)+removed+after.slice(start+inserted.length)===a};}
function fresh(path:string){const source=readFileSync(resolve(root,path),"utf8");return{path,source,sha256:sha(source)};}
function replaceOnce(body:string,before:string,after:string){if(body.split(before).length!==2)throw Error("Exact scene span refused "+before.slice(0,80));return body.replace(before,after);}
const cfg='#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]',portPath=canonical+"/🎯️targets/🧊️wgpu/🦀️.rs",testPath=canonical+"/🧪️tests/🎬️scene-target/🦀️.rs",fixturePath=canonical+"/🧫️fixtures/🎬️scene-target/🔣️.json",schemaPath=canonical+"/🧬️schema/🎬️scene-target/🔣️.json";
const port=`//! 🎬️ Canvas-owned full-fidelity scene rasterization on an admitted WGPU target.
use crate::{Color, Scene};
pub use vello::wgpu::{Device, Queue, TextureView};

/// 🎨️ Exact caller-owned surface extent and straight-alpha clear color.
pub struct SceneRenderOptions {
    pub background: [f32; 4],
    pub width: u32,
    pub height: u32,
}

/// 🖥️ Renderer retained by the caller's existing allocation and retirement cursor.
#[repr(transparent)]
pub struct SceneRenderer {
    renderer: vello::Renderer,
}

impl SceneRenderer {
    /// 🧱️ Allocates the same area-only renderer after the caller admits its physical allocation.
    pub fn new(device: &Device) -> Result<Self, String> {
        let renderer = vello::Renderer::new(device, vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None })
            .map_err(|error| format!("vello renderer: {error:?}"))?;
        Ok(Self { renderer })
    }

    /// 🎬️ Replays the complete scene onto the exact admitted target without readback or command truncation.
    pub fn render(&mut self, device: &Device, queue: &Queue, scene: &Scene, target: &TextureView, options: SceneRenderOptions) -> Result<(), String> {
        let params = vello::RenderParams { base_color: Color::new(options.background).to_peniko(), width: options.width, height: options.height, antialiasing_method: vello::AaConfig::Area };
        let encoded = scene.vello_scene();
        self.renderer.render_to_texture(device, queue, &encoded, target, &params).map_err(|error| format!("vello render: {error:?}"))
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🎬️scene-target/🦀️.rs"]
mod scene_target_tests;
`;
let canvasAfter=replaceOnce(canvasRow.after,"pub fn vello_scene(&self) -> backend::Scene", "pub(crate) fn vello_scene(&self) -> backend::Scene");
canvasAfter+=`\n${cfg}\n#[path = "🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod raster_target;\n`;
const engineBefore=readFileSync(resolve(root,enginePath),"utf8");let engineAfter=replaceOnce(engineBefore,"use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};","use canvas::raster_target::{SceneRenderOptions, SceneRenderer};");engineAfter=replaceOnce(engineAfter,"renderer: Option<Renderer>","renderer: Option<SceneRenderer>");
const allocate='Renderer::new(gpu.device(), RendererOptions { use_cpu: false, antialiasing_support: AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None })\n                        .map_err(|error| format!("vello renderer: {error:?}"))?';engineAfter=replaceOnce(engineAfter,allocate,"SceneRenderer::new(gpu.device())?");
const render='let params = RenderParams { base_color: packet.clear, width: build.width, height: build.height, antialiasing_method: AaConfig::Area };\n                let vello_scene = packet.scene.vello_scene();\n                renderer.render_to_texture(gpu.device(), gpu.queue(), &vello_scene, view, &params).map_err(|error| format!("vello render: {error:?}"))?;';engineAfter=replaceOnce(engineAfter,render,'let options = SceneRenderOptions { background: packet.clear.components, width: build.width, height: build.height };\n                renderer.render(gpu.device(), gpu.queue(), &packet.scene, view, options)?;');
const operations=["fill","stroke","image","layer","clip","pop","fragment"],fixture={contract:"canvas-full-fidelity-scene-target/v1",transform:[1.000125,0.25,-0.125,1.5,0.000125,-0.000375],outer:[2,0,0,0.5,3,-4],expectedComposed:[2.00025,0.125,-0.25,0.75,3.00025,-4.0001875],rectangle:[0.000125,-0.000375,4.125,8.0625],color:[0.25,0.5,0.75,0.5],pixels:[255,0,0,255,0,255,0,128],cases:operations.map(id=>({id,operation:id,expectedCommands:id==="layer"||id==="clip"?2:1}))};
const schema={$schema:"https://json-schema.org/draft/2020-12/schema",$id:"https://schemas.semio.tech/framework/ui/canvas/scene-target",type:"object",additionalProperties:false,required:Object.keys(fixture),properties:Object.fromEntries(Object.entries(fixture).map(([key,value])=>[key,{const:value}]))};
const fixtureLiteral=relative(dirname(testPath),fixturePath).split("\\").join("/");
const tests=`//! 🎬️ Full-fidelity command streams match the independent Vello encoding.
use crate::renderer::{BlendMode, Cap, Color, FillRule, RasterImage, Scene, SceneCommand, Stroke};
use geometry::{Affine, Rect};
use std::sync::Arc;

#[test]
fn full_scene_command_streams_match_vello_without_draw_list_rounding() {
    assert_eq!(std::mem::size_of::<super::SceneRenderer>(), std::mem::size_of::<vello::Renderer>());
    assert_eq!(std::mem::align_of::<super::SceneRenderer>(), std::mem::align_of::<vello::Renderer>());
    assert_eq!(std::mem::size_of::<Option<super::SceneRenderer>>(), std::mem::size_of::<Option<vello::Renderer>>());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("${fixtureLiteral}")).unwrap();
    let coefficients = |name: &str| { let values = fixture[name].as_array().unwrap(); std::array::from_fn::<_, 6, _>(|index| values[index].as_f64().unwrap()) };
    let transform = Affine::new(coefficients("transform"));
    let outer = Affine::new(coefficients("outer"));
    for (actual, expected) in (outer * transform).as_coeffs().into_iter().zip(coefficients("expectedComposed")) { assert!((actual - expected).abs() < 1e-12); }
    let rect = fixture["rectangle"].as_array().unwrap();
    let rect = Rect::new(rect[0].as_f64().unwrap(), rect[1].as_f64().unwrap(), rect[2].as_f64().unwrap(), rect[3].as_f64().unwrap());
    let raw = fixture["color"].as_array().unwrap();
    let rgba: [f32; 4] = std::array::from_fn(|index| raw[index].as_f64().unwrap() as f32);
    let color = Color::new(rgba);
    let reference_color = vello::peniko::Color::new(rgba);
    let reference_rect = kurbo::Rect::new(rect.x0, rect.y0, rect.x1, rect.y1);
    let reference_transform = kurbo::Affine::new(transform.as_coeffs());
    let pixels = fixture["pixels"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect::<Vec<_>>();
    for case in fixture["cases"].as_array().unwrap() {
        let mut actual = Scene::new();
        let mut reference = vello::Scene::new();
        match case["operation"].as_str().unwrap() {
            "fill" => { actual.fill(FillRule::EvenOdd, transform, color, Some(outer), &rect); reference.fill(vello::peniko::Fill::EvenOdd, reference_transform, reference_color, Some(kurbo::Affine::new(outer.as_coeffs())), &reference_rect); }
            "stroke" => {
                let mut stroke = Stroke::new(1.125); stroke.set_dash_pattern(vec![1.5, 0.25]); stroke.set_start_cap(Cap::Square); stroke.set_end_cap(Cap::Butt);
                let mut independent = kurbo::Stroke::new(1.125); independent.join = kurbo::Join::Round; independent.miter_limit = 4.0; independent.start_cap = kurbo::Cap::Square; independent.end_cap = kurbo::Cap::Butt; independent.dash_pattern = [1.5, 0.25].into_iter().collect(); independent.dash_offset = 0.0;
                actual.stroke(&stroke, transform, color, Some(outer), &rect); reference.stroke(&independent, reference_transform, reference_color, Some(kurbo::Affine::new(outer.as_coeffs())), &reference_rect);
            }
            "image" => {
                let image = RasterImage::rgba8(2, 1, Arc::new(pixels.clone())); actual.draw_image(&image, transform);
                let converted = image.to_peniko(); assert_eq!(converted.data.data(), pixels.as_slice()); assert_eq!((converted.width, converted.height), (2, 1)); assert_eq!(converted.format, vello::peniko::ImageFormat::Rgba8); assert_eq!(converted.alpha_type, vello::peniko::ImageAlphaType::Alpha);
                let independent = vello::peniko::ImageData { data: vello::peniko::Blob::new(Arc::new(pixels.clone())), format: vello::peniko::ImageFormat::Rgba8, alpha_type: vello::peniko::ImageAlphaType::Alpha, width: 2, height: 1 };
                reference.draw_image(&vello::peniko::ImageBrush::new(independent), reference_transform);
            }
            "layer" => { actual.push_layer(FillRule::EvenOdd, BlendMode::Multiply, 0.5, transform, &rect); actual.pop_layer(); reference.push_layer(vello::peniko::Fill::EvenOdd, vello::peniko::Mix::Multiply, 0.5, reference_transform, &reference_rect); reference.pop_layer(); }
            "clip" => { actual.push_clip_layer(FillRule::NonZero, transform, &rect); actual.pop_layer(); reference.push_clip_layer(vello::peniko::Fill::NonZero, reference_transform, &reference_rect); reference.pop_layer(); }
            "pop" => { actual.pop_layer(); reference.pop_layer(); }
            "fragment" => {
                let mut fragment = vello::Scene::new(); fragment.fill(vello::peniko::Fill::NonZero, kurbo::Affine::IDENTITY, reference_color, None, &reference_rect);
                reference.append(&fragment, Some(reference_transform)); actual.0.push(SceneCommand::VelloFragment { scene: Arc::new(fragment), transform });
            }
            _ => unreachable!(),
        }
        assert_eq!(actual.command_len(), case["expectedCommands"].as_u64().unwrap() as usize);
        let encoded = actual.vello_scene(); let a = encoded.encoding(); let b = reference.encoding();
        assert_eq!(a.path_data, b.path_data); assert_eq!(a.draw_data, b.draw_data);
        assert_eq!(format!("{:?}", a.path_tags), format!("{:?}", b.path_tags)); assert_eq!(format!("{:?}", a.draw_tags), format!("{:?}", b.draw_tags));
        assert_eq!(format!("{:?}", a.transforms), format!("{:?}", b.transforms)); assert_eq!(format!("{:?}", a.styles), format!("{:?}", b.styles));
        assert_eq!((a.n_paths, a.n_path_segments, a.n_clips, a.n_open_clips, a.flags), (b.n_paths, b.n_path_segments, b.n_clips, b.n_open_clips, b.flags));
        assert_eq!(a.resources.patches.len(), b.resources.patches.len());
        println!("[DEBUG] full_scene_port {} commands={} paths={} clips={} precision_retained=true", case["id"], actual.command_len(), a.n_paths, a.n_clips);
    }
}
`;
const {default:Parser}=await import("web-tree-sitter");await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));
function descendants(node:any,type:string):any[]{return(node.type===type?[node]:[]).concat(node.namedChildren.flatMap((child:any)=>descendants(child,type)));}
function errors(node:any):any[]{return(node.type==="ERROR"||node.isMissing()?[{type:node.type,text:node.text}]:[]).concat(node.children.flatMap(errors));}
function method(body:string,name:string){const parsed=parser.parse(body),found=descendants(parsed.rootNode,"function_item").filter(node=>node.childForFieldName("name")?.text===name),result=found.map(node=>node.text);parsed.delete();return result;}
const replayBefore=method(canvasRow.after,"replay_into"),replayAfter=method(canvasAfter,"replay_into");if(!isDeepStrictEqual(replayBefore,replayAfter)||replayAfter.length!==1)throw Error("Original full scene replay algorithm changed");
const canvasSuffix=`\n${cfg}\n#[path = "🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod raster_target;\n`;if(!canvasAfter.endsWith(canvasSuffix)||replaceOnce(canvasAfter.slice(0,-canvasSuffix.length),"pub(crate) fn vello_scene(&self) -> backend::Scene","pub fn vello_scene(&self) -> backend::Scene")!==canvasRow.after)throw Error("Canvas outside exact visibility and target mount changed");
const authoredCanvasTree=parser.parse(canvasRow.after),authoredCanvasErrors=errors(authoredCanvasTree.rootNode);authoredCanvasTree.delete();const successorCanvasTree=parser.parse(canvasAfter),successorCanvasErrors=errors(successorCanvasTree.rootNode);successorCanvasTree.delete();if(!isDeepStrictEqual(authoredCanvasErrors,successorCanvasErrors))throw Error("Introduced Canvas grammar error");
const rows=[pair(canvasPath,canvasRow.before,canvasAfter),pair(portPath,null,port),pair(testPath,null,tests),pair(fixturePath,null,JSON.stringify(fixture,null,2)+"\n"),pair(schemaPath,null,JSON.stringify(schema,null,2)+"\n"),pair(enginePath,engineBefore,engineAfter)],grammar=rows.filter(row=>row.path.endsWith(".rs")).map(row=>{const after=parser.parse(row.after),afterErrors=errors(after.rootNode);after.delete();const before=row.before===null?null:parser.parse(row.before),beforeErrors=before?errors(before.rootNode):[];before?.delete();return{path:row.path,beforeErrors,afterErrors};});parser.delete();
const {default:Ajv}=await import("ajv/dist/2020.js"),ajv=new Ajv({strict:true,allErrors:true}),validate=ajv.compile(schema);if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const {default:Decimal}=await import("decimal.js"),a=fixture.outer.map(value=>new Decimal(value)),b=fixture.transform.map(value=>new Decimal(value)),reference=[a[0].mul(b[0]).add(a[2].mul(b[1])),a[1].mul(b[0]).add(a[3].mul(b[1])),a[0].mul(b[2]).add(a[2].mul(b[3])),a[1].mul(b[2]).add(a[3].mul(b[3])),a[0].mul(b[4]).add(a[2].mul(b[5])).add(a[4]),a[1].mul(b[4]).add(a[3].mul(b[5])).add(a[5])].map(value=>value.toNumber());if(!isDeepStrictEqual(reference,fixture.expectedComposed))throw Error("Independent Decimal affine oracle refused");
const retainedFunctions=["advance","retained_bytes","retirement_backing_bytes","append_range","render_frame"].map(name=>({name,before:methodSource(canvasRow.after,name),after:methodSource(canvasAfter,name)}));
function methodSource(body:string,name:string){return body.split("\n").filter(line=>line.includes("fn "+name+"(")).join("\n");}
const contexts=[fresh("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs"),fresh("🧰️framework/🔨️modules/🖌️raster/🦀️.rs"),fresh("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml")];
writeFileSync(output,JSON.stringify({at:new Date().toISOString(),prior:{path:priorPath,sha256:sha(priorSource)},rows,contexts,grammar,authoredCanvasGrammar:{before:authoredCanvasErrors,after:successorCanvasErrors,exactConserved:true},fixtureProof:{ajv:true,decimalAffine:reference,cases:fixture.cases.length},sourceConservation:{fullReplayFunctionExact:true,entireCanvasExceptVisibilityAndTargetMountExact:true,retainedFunctions,engineOutsideFourOwnedSpansExact:true,allPhaseAndAdmissionCallsRetained:true},technologyBoundary:{module:"raster_target",publicDomainSceneBackendReturnRemoved:true,explicitReexports:["Device","Queue","TextureView"],actualNecessity:"Existing Engine target supplies its already admitted device/queue/view; these types are confined to the WGPU target. No new device, texture, readback, CPU fallback, or command truncation.",rendererLayout:"Single private original vello::Renderer field; actual native layout and resource retirement remain unexecuted"},mountReady:false,nativeExecuted:false,sourceWritesOutsideTicket:false,remaining:["Compose original physical owner45/4 with current providers/callers/routes and shared GUI before publication.","Native independent Vello encoding law is authored only and does not prove physical GPU rendering.","DAG seed/VCS provider ownership is a separate held successor."]},null,2));console.log(JSON.stringify({output,rows:rows.length,cases:fixture.cases.length,affineOracle:reference,grammar:grammar.map(row=>({path:row.path,before:row.beforeErrors.length,after:row.afterErrors.length})),authoredCanvasGrammarExact:true,fullReplayExact:true,nativeExecuted:false,sourceWritesOutsideTicket:false}));
