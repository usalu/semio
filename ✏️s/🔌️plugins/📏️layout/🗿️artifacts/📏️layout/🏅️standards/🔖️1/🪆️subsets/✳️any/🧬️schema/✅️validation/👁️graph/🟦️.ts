import type {LayoutArtifact,ParagraphStyle,CharacterStyle,TextStory,ParentPage,Spread,Page,Frame,LayoutRect,TextStyleRun,LayoutDrawingMember,DrawingSourceAddress} from '../../🟦️.ts';
export type GraphField<T>=T extends readonly unknown[]?{readonly[K in keyof T]:GraphField<T[K]>}:T extends object?{readonly[K in keyof T]:GraphField<T[K]>}:T;
export type FrameDescriptor=Pick<Frame,'id'|'layerId'|'bounds'>&(Pick<Extract<Frame,{kind:'rect'}>,'kind'|'fill'|'stroke'>|Pick<Extract<Frame,{kind:'text'}>,'kind'|'storyId'|'threadNext'|'columns'|'inset'>|Pick<Extract<Frame,{kind:'image'}>,'kind'|'linkId'>);
export type FrameRead=GraphField<FrameDescriptor>;
export type ContentOwner={kind:'parent'|'page';index:number};
export type ParagraphRead=GraphField<Pick<ParagraphStyle,'id'|'name'|'fontFamily'|'fontSize'|'fontWeight'|'leading'|'tracking'|'alignment'>>;
export type CharacterRead=GraphField<Pick<CharacterStyle,'id'|'fontSize'|'fontWeight'|'tracking'|'color'>>;
export type StoryRead=GraphField<Pick<TextStory,'id'|'content'>>;
export type ParentRead=GraphField<Pick<ParentPage,'id'|'name'|'width'|'height'>>;
export type SpreadRead=GraphField<Pick<Spread,'id'|'name'>>;
export type PageRead=GraphField<Pick<Page,'id'|'spreadId'|'parentPageId'|'width'|'height'|'margins'|'columns'>>;
export interface LayoutGraphRead{
 schema():string;grid():GraphField<LayoutArtifact['grid']>;paragraphCount():number;paragraph(row:number):ParagraphRead;characterCount():number;character(row:number):CharacterRead;
 storyCount():number;story(row:number):StoryRead;runCount(story:number):number;run(story:number,row:number):GraphField<TextStyleRun>;linkCount():number;linkId(row:number):string;
 parentCount():number;parent(row:number):ParentRead;spreadCount():number;spread(row:number):SpreadRead;spreadPageCount(spread:number):number;spreadPage(spread:number,row:number):string;
 pageCount():number;page(row:number):PageRead;guideCount(page:number):number;guide(page:number,row:number):GraphField<LayoutRect>;
 layerCount(owner:ContentOwner):number;layerId(owner:ContentOwner,row:number):string;cachedLayerCount(owner:ContentOwner):number;cachedLayer(owner:ContentOwner,row:number):string;
 memberCount(owner:ContentOwner,layer:number):number;member(owner:ContentOwner,layer:number,row:number):string;frameCount(owner:ContentOwner):number;frame(owner:ContentOwner,row:number):FrameRead;
 overrideCount(page:number):number;pageOverride(page:number,row:number):GraphField<Page['overrides'][number]>;drawing(source:DrawingSourceAddress):GraphField<LayoutDrawingMember>|undefined;
}
/** 👁️ A borrow-only original adapter confers no source, grant or publication authority. */
export class SnapshotGraphRead implements LayoutGraphRead{
 constructor(readonly original:LayoutArtifact){}
 private content(owner:ContentOwner){return owner.kind==='parent'?this.original.parentPages[owner.index]!:this.original.pages[owner.index]!}
 schema(){return this.original.schema}grid():GraphField<LayoutArtifact['grid']>{return this.original.grid}
 paragraphCount(){return this.original.paragraphStyles.length}paragraph(row:number):ParagraphRead{return this.original.paragraphStyles[row]!}characterCount(){return this.original.characterStyles.length}character(row:number):CharacterRead{return this.original.characterStyles[row]!}
 storyCount(){return this.original.stories.length}story(row:number):StoryRead{return this.original.stories[row]!}runCount(story:number){return this.original.stories[story]!.styleRuns.length}run(story:number,row:number):GraphField<TextStyleRun>{return this.original.stories[story]!.styleRuns[row]!}linkCount(){return this.original.links.length}linkId(row:number){return this.original.links[row]!.id}
 parentCount(){return this.original.parentPages.length}parent(row:number):ParentRead{return this.original.parentPages[row]!}spreadCount(){return this.original.spreads.length}spread(row:number):SpreadRead{return this.original.spreads[row]!}spreadPageCount(spread:number){return this.original.spreads[spread]!.pageIds.length}spreadPage(spread:number,row:number){return this.original.spreads[spread]!.pageIds[row]!}
 pageCount(){return this.original.pages.length}page(row:number):PageRead{return this.original.pages[row]!}guideCount(page:number){return this.original.pages[page]!.guides.length}guide(page:number,row:number):GraphField<LayoutRect>{return this.original.pages[page]!.guides[row]!}
 layerCount(owner:ContentOwner){return this.content(owner).layers.length}layerId(owner:ContentOwner,row:number){return this.content(owner).layers[row]!.id}cachedLayerCount(owner:ContentOwner){return this.content(owner).layerIds.length}cachedLayer(owner:ContentOwner,row:number){return this.content(owner).layerIds[row]!}
 memberCount(owner:ContentOwner,layer:number){return this.content(owner).layers[layer]!.objectIds.length}member(owner:ContentOwner,layer:number,row:number){return this.content(owner).layers[layer]!.objectIds[row]!}frameCount(owner:ContentOwner){return this.content(owner).frames.length}frame(owner:ContentOwner,row:number):FrameRead{return this.content(owner).frames[row]!}
 overrideCount(page:number){return this.original.pages[page]!.overrides.length}pageOverride(page:number,row:number):GraphField<Page['overrides'][number]>{return this.original.pages[page]!.overrides[row]!}drawing(source:DrawingSourceAddress):GraphField<LayoutDrawingMember>|undefined{return source.kind==='document'?this.original.backgroundDrawing:this.original.pages[source.page]!.backgroundDrawing}
}

export {FinalGraphSourceBindings} from './🔗️bindings/🟦️.ts';
