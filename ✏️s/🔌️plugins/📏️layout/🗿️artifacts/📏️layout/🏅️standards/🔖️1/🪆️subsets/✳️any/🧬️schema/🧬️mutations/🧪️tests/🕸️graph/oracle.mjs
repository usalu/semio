/** 🧪️ Test-only independent Unicode annotation and RFC 6902 graph oracle. */
import assert from 'node:assert/strict';
import jsonPatch from 'fast-json-patch';
import canvas from '@napi-rs/canvas';
const {DOMMatrix}=canvas;
import { graphValid, normalizedParent } from '../✅️validation/oracle.mjs';
const bytes = value => new TextEncoder().encode(value).length;
const copy = value => JSON.parse(JSON.stringify(value));

/** ✂️ Tracks styled scalar tokens, independently of native interval arithmetic. */
export function spliceStory(story, payload) {
  const chars = [...story.content];
  const end = payload.offset + payload.delete;
  if (!Number.isSafeInteger(end) || payload.offset > chars.length || end > chars.length) return null;
  const offsets = [0];
  for (const char of chars) offsets.push(offsets.at(-1) + bytes(char));
  const startByte = offsets[payload.offset];
  const endByte = offsets[end];
  const inserted = [...payload.insert];
  const content = [...chars.slice(0, payload.offset), ...inserted, ...chars.slice(end)].join('');
  const output = [...chars.slice(0, payload.offset).map((char, at) => ({char, origin: offsets[at]})), ...inserted.map(char => ({char, origin: null})), ...chars.slice(end).map((char, at) => ({char, origin: offsets[end + at]}))];
  const styleRuns = [];
  for (const run of story.styleRuns) {
    if (run.start === run.end) {
      const anchor = run.start < startByte ? run.start : run.start >= endByte ? run.start - (endByte - startByte) + bytes(payload.insert) : startByte + bytes(payload.insert);
      styleRuns.push({...run, start: anchor, end: anchor});
      continue;
    }
    const inherit = run.start <= startByte && startByte < run.end || startByte === bytes(story.content) && run.start < startByte && run.end === startByte;
    let at = 0;
    const selected = [];
    for (const token of output) {
      const length = bytes(token.char);
      if (token.origin === null ? inherit : run.start <= token.origin && token.origin < run.end) selected.push([at, at + length]);
      at += length;
    }
    if (selected.length) styleRuns.push({...run, start: selected[0][0], end: selected.at(-1)[1]});
  }
  return {...story, content, styleRuns};
}

/** ⚖️ Publishes projected semantic edits only when their resulting independent graph is valid. */
export function projectMutation(before, mutation) {
  const after = copy(before);
  const [kind, value] = Object.entries(mutation)[0];
  const page = after.pages.find(row => row.id === (value.pageId ?? value.id));
  const derive = page => { for (const layer of page.layers) layer.objectIds = page.frames.filter(frame => frame.layerId === layer.id).map(frame => frame.id); };
  if (kind === 'ReplaceDocument') {
    return graphValid(value.document) ? copy(value.document) : copy(before);
  } else if (kind === 'CreateParentPage') {
    if(value.index!==null&&value.index>after.parentPages.length)return copy(before);
    after.parentPages.splice(value.index??after.parentPages.length,0,normalizedParent(value.page));
  } else if (kind === 'SetParentPageContent') {
    const parent=after.parentPages.find(parent=>parent.id===value.id);if(!parent)return copy(before);
    const replacement=normalizedParent({...parent,layers:copy(value.layers),frames:copy(value.frames)});Object.assign(parent,replacement);
    for(const page of after.pages.filter(page=>page.parentPageId===parent.id))page.overrides=page.overrides.filter(change=>parent.frames.some(frame=>frame.id===change.objectId));
  } else if (kind === 'UpdateTextFrame') {
    const frame = page?.frames.find(frame => frame.id === value.frameId);
    if (!frame || frame.kind !== 'text') return copy(before);
    Object.assign(frame, {storyId:value.storyId, threadNext:value.threadNext, inset:{x:value.insetX,y:value.insetY,w:value.insetWidth,h:value.insetHeight}});
  } else if (kind === 'DeleteFrame') {
    if (!page?.frames.some(frame => frame.id === value.frameId)) return copy(before);
    page.frames = page.frames.filter(frame => frame.id !== value.frameId); derive(page);
  } else if (kind === 'CreateFrame') {
    if (!page || (value.index !== null && value.index > page.frames.length)) return copy(before);
    const frame = copy(value.frame); if(value.layerId !== null) frame.layerId = value.layerId;
    page.frames.splice(value.index ?? page.frames.length, 0, frame); derive(page);
  } else if (kind === 'CreatePage') {
    const spread = after.spreads.find(row => row.id === value.page.spreadId);
    if (!spread || (value.index !== null && value.index > after.pages.length)) return copy(before);
    after.pages.splice(value.index ?? after.pages.length, 0, copy(value.page)); spread.pageIds.push(value.page.id);
  } else if (kind === 'DeletePage') {
    if(!page) return copy(before);
    after.pages = after.pages.filter(row => row.id !== value.id);
    for (const spread of after.spreads) spread.pageIds = spread.pageIds.filter(id => id !== value.id);
  } else if (kind === 'CreateStory') {
    if(value.index !== null && value.index > after.stories.length) return copy(before);
    after.stories.splice(value.index ?? after.stories.length, 0, copy(value.story));
  } else if (kind === 'CreateLink') {
    if(value.index !== null && value.index > after.links.length) return copy(before);
    after.links.splice(value.index ?? after.links.length, 0, copy(value.link));
  } else if (kind === 'ChangeFrameColumns') {
    const frame=page?.frames.find(frame=>frame.id===value.frameId);if(!frame||frame.kind!=='text')return copy(before);frame.columns=value.newColumns;
  } else if (kind === 'UpdatePageColumns') {
    if(!page)return copy(before);page.columns={count:value.count,gutter:value.gutter};
  } else if (kind === 'SetPageParent') {
    if(!page)return copy(before);page.parentPageId=value.parentPageId;
  } else if (['DragFrames','RotateFrames','ScaleFrames'].includes(kind)) {
    if(!page)return copy(before);
    for(const frame of page.frames.filter(frame=>value.targets.includes(frame.id))) {
      const box=frame.bounds;let matrix=new DOMMatrix();
      if(kind==='DragFrames')matrix=matrix.translate(value.dx,value.dy);
      else if(kind==='RotateFrames')matrix=matrix.translate(value.pivotX,value.pivotY).rotate(value.angle*180/Math.PI).translate(-value.pivotX,-value.pivotY);
      else matrix=matrix.translate(value.pivotX,value.pivotY).scale(value.sx,value.sy).translate(-value.pivotX,-value.pivotY);
      const center=matrix.transformPoint({x:box.x+box.w/2,y:box.y+box.h/2,z:0,w:1});
      if(kind==='ScaleFrames'){box.w*=value.sx;box.h*=value.sy;}
      box.x=center.x-box.w/2;box.y=center.y-box.h/2;
      if(kind==='RotateFrames')box.rotation+=value.angle;
    }
  } else if (kind === 'EditStory') {
    const at = after.stories.findIndex(row => row.id === value.id);
    if (at < 0) return copy(before);
    const story = spliceStory(after.stories[at], value);
    if (!story) return copy(before);
    after.stories[at] = story;
  } else throw new Error('Unsupported oracle witness: '+kind);
  return graphValid(after) ? after : copy(before);
}

/** 🔍️ Checks portable outputs with Ajv, Kahn ordering, Unicode tokens and external JSON Patch. */
export function verifySemanticGraph(corpus) {
  let accepted=0, refused=0;
  for (const vector of corpus.cases) {
    assert.equal(graphValid(vector.before), true, vector.name+': precondition');
    assert.equal(graphValid(vector.after), true, vector.name+': published graph');
    const patched = jsonPatch.applyPatch(copy(vector.before), vector.patch, true, false).newDocument;
    assert.equal(jsonPatch.compare(patched, copy(vector.after)).length, 0, vector.name+': RFC 6902 output');
    assert.equal(jsonPatch.compare(projectMutation(vector.before, vector.mutation), copy(vector.after)).length, 0, vector.name+': independent semantic projection');
    assert.equal(JSON.stringify(vector.before) !== JSON.stringify(vector.after), vector.accepted, vector.name+': acceptance');
    if(vector.accepted) accepted++; else refused++;
  }
  for(const vector of corpus.wordCases??[]) {
    assert.equal(graphValid(vector.before),true,vector.name+': precondition');
    const mutation=copy(vector.mutation),view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt('0x'+vector.binary64));
    Object.values(mutation)[0][vector.field]=view.getFloat64(0);
    assert.equal(jsonPatch.compare(projectMutation(vector.before,mutation),copy(vector.before)).length,0,vector.name+': nonfinite refusal');
  }
  return {cases:corpus.cases.length,accepted,refused,wordCases:(corpus.wordCases??[]).length};
}
