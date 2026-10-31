"""🎞️ Decode every final native movie frame and retain visual inspection sheets."""
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys
import time
import av
import numpy as np
from PIL import Image, ImageDraw
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
RENDERS=Path(sys.argv[1]) if len(sys.argv)>1 else TICKET/'🗑️generated/native-r92/final'
OUT=TICKET/'🗑️generated/native-r92/movie-qa'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
IDS=['bim_information','bim_objects','bim_representations','bim_models','bim_exchange','bim_consistency']
VIDEOS=[v for v in runtime.load_manifest(source_ids=IDS) if v['id'] in IDS]
ROWS=[]
CANCEL=TICKET/'native-r92-cancel'
def check_cancel():
    """🛑 Keep full media decoding interruptible between frames."""
    if CANCEL.exists():
        raise KeyboardInterrupt('Native movie QA cancellation requested')
def digest(path):
    """🔐 Hash files incrementally without loading complete movie bytes."""
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):
            check_cancel()
            h.update(block)
    return h.hexdigest()
def save_report(success):
    """🧾 Persist incremental frame-clock and current-source provenance."""
    (OUT/'result.json').write_text(json.dumps({'success':success,'expected':24,'completed':len(ROWS),'movies':ROWS},indent=2),encoding='utf-8')
def decode_audio(movie,seconds):
    """🔊 Decode the complete published audio track and report its actual finite clock and signal presence."""
    with av.open(str(movie)) as container:
        assert len(container.streams.audio)==1,('Missing or ambiguous final audio track',movie)
        stream=container.streams.audio[0];rate=stream.codec_context.sample_rate;assert rate>0
        samples=0;frames=0;nonzero=False;previous=None
        for frame in container.decode(stream):
            check_cancel();assert frame.sample_rate==rate
            stamp=frame.pts*frame.time_base
            if previous is not None:assert stamp>previous,(movie,'audio timestamp')
            previous=stamp;samples+=frame.samples;frames+=1
            values=frame.to_ndarray();assert np.isfinite(values).all()
            nonzero=nonzero or bool(values.any())
        assert frames and abs(samples/rate-seconds)<=max(.1,2*stream.codec_context.frame_size/rate)
        return {'frames':frames,'samples':samples,'sample_rate':rate,'seconds':samples/rate,'has_signal':nonzero}

def authored_targets(video,language,audience,settings):
    """🎯 Inspect every authored segment and both context transitions against exact published chapter clocks."""
    collection=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️')
    plan=runtime.segment_plan(VIDEOS);offset=0;targets=set();coverage=[]
    for chapter_index,chapter in enumerate(video['chapters']):
        check_cancel();published=collection.chapter_output(RENDERS/language/audience/'rendered',chapter['scene'],settings)
        clock=json.loads(published.with_suffix('.chapter.json').read_text(encoding='utf8'));assert clock['fps']==30
        targets.update((offset+clock['focus_frames']//2,offset+clock['focus_frames']+clock['body_frames']+clock['restore_frames']//2))
        elapsed=0
        for job in (job for job in plan if job['video_id']==video['id'] and job['chapter_index']==chapter_index):
            source=RENDERS/language/audience/'s'/str(job['source_ordinal'])/'v'/(job['segment']['scene']+'.mp4')
            selected=runtime.excerpt_segment(source,job['segment'],settings,concat_only=True)
            with av.open(str(selected)) as clip:
                stream=clip.streams.video[0];frames=stream.frames;assert frames and stream.average_rate==30
            frame=offset+clock['focus_frames']+elapsed+(frames-1)//2;targets.add(frame)
            coverage.append({'chapter':chapter['scene'],'segment':job['segment'],'frames':frames,'sample_frame':frame});elapsed+=frames
        assert elapsed==clock['body_frames'],(chapter['scene'],elapsed,clock)
        offset+=clock['frames']
    return targets,coverage,offset

try:
    for language in ('en','de'):
        for audience in ('speaker','spectator'):
            for video in VIDEOS:
                check_cancel()
                movie=RENDERS/language/audience/'rendered'/f"{video['output']}_{audience}_{language}_1920x1080p30.mp4"
                sidecar=movie.with_suffix('.render.json')
                state=json.loads(sidecar.read_text(encoding='utf-8'))
                assert runtime.state_inputs_current(state),f'Stale render sources: {movie}'
                assert state['content']['video']==video['id']
                settings=state['content']['settings']
                assert settings['language']==language and settings['audience']==audience
                assert settings['resolution']=='1920,1080' and float(settings['fps'])==30
                before=digest(movie)
                count=0
                previous=None
                samples={}
                with av.open(str(movie)) as container:
                    stream=container.streams.video[0]
                    assert (stream.width,stream.height)==(1920,1080)
                    assert stream.average_rate==30
                    expected=stream.frames
                    if not expected:
                        raise AssertionError(f'Missing native frame count: {movie}')
                    authored,coverage,published_frames=authored_targets(video,language,audience,settings)
                    assert published_frames==expected,(movie,published_frames,expected)
                    targets={round((expected-1)*fraction/8) for fraction in range(9)}|authored
                    assert all(0<=frame<expected for frame in targets)
                    for frame in container.decode(stream):
                        check_cancel()
                        stamp=frame.pts*frame.time_base
                        if previous is not None:
                            assert stamp-previous==1/stream.average_rate,(movie,count,stamp,previous)
                        previous=stamp
                        assert (frame.width,frame.height)==(1920,1080)
                        if count in targets:
                            image=frame.to_image().convert('RGB')
                            image.save(OUT/f"{video['id']}-{language}-{audience}-{count:06}.png")
                            image.thumbnail((480,270));samples[count]=image.copy()
                        count+=1
                        if count%900==0:
                            print(f'[DEBUG] Decoded {language}/{audience}/{video["id"]}: {count}/{expected}',flush=True)
                    assert count==expected,(movie,count,expected)
                audio=decode_audio(movie,count/30)
                assert digest(movie)==before,f'Movie changed while decoding: {movie}'
                assert runtime.state_inputs_current(state),f'Source changed while decoding: {movie}'
                sheets=[];ordered=list(samples.items())
                for page,start in enumerate(range(0,len(ordered),9),1):
                    sheet=Image.new('RGB',(1440,870),'#e8eef0');draw=ImageDraw.Draw(sheet)
                    for index,(number,image) in enumerate(ordered[start:start+9]):
                        x=(index%3)*480;y=(index//3)*290;sheet.paste(image,(x,y))
                        draw.text((x+8,y+272),f'{video["id"]} {language} {audience} {number/30:.2f}s',fill='#132e34')
                    sheetpath=OUT/f'{video["id"]}-{language}-{audience}-contact-{page}.png';sheet.save(sheetpath);sheets.append(str(sheetpath))
                ROWS.append({'video':video['id'],'language':language,'audience':audience,'path':str(movie),'sha256':before,'frames':count,'fps':30,'resolution':[1920,1080],'seconds':count/30,'sidecar_sha256':digest(sidecar),'current_sources':True,'contact_sheets':sheets,'authored_segment_coverage':coverage,'audio':audio,'sample_frames':list(samples)})
                save_report(False)
                print(f'[DEBUG] Complete native movie QA {language}/{audience}/{video["id"]}: {count} frames',flush=True)
    assert len(ROWS)==24
    save_report(True)
except BaseException:
    save_report(False)
    raise
