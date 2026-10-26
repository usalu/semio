"""🖼️ Inspect early, middle and late frames of each complete rendered lesson."""
import argparse
import json
from pathlib import Path
from fractions import Fraction
import subprocess


def parse_args(argv=None):
    """📐 Select the same explicit format profile used by the independent verifier."""
    parser=argparse.ArgumentParser()
    parser.add_argument("root",type=Path)
    parser.add_argument("--ffprobe",required=True)
    parser.add_argument("--output",required=True,type=Path)
    parser.add_argument("--teaching-root",required=True,type=Path)
    parser.add_argument("--width",type=int,default=854)
    parser.add_argument("--height",type=int,default=480)
    parser.add_argument("--fps",type=float,default=15)
    args=parser.parse_args(argv)
    if args.width <= 0 or args.height <= 0 or args.width % 2 or args.height % 2 or args.fps <= 0:
        parser.error("Expected positive even width/height and positive fps")
    return args


def movie_suffix(args):
    """🏷️ Match the renderer explicit-resolution filename convention."""
    dimensions = str(args.height) if (args.width, args.height) == (854, 480) else f"{args.width}x{args.height}"
    return f"{dimensions}p{args.fps:g}"


def main():
    """🖼️ Decode early, middle and late frames at the verified output dimensions."""
    args=parse_args()
    import av
    from PIL import Image, ImageDraw
    args.output.mkdir(parents=True,exist_ok=True)
    movies=sorted((args.root / "rendered").glob("*.mp4"))
    manifest=json.loads((args.teaching_root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
    expected={video["output"]+f"_{movie_suffix(args)}.mp4" for video in manifest}
    assert len(movies)==5 and {movie.name for movie in movies}==expected, movies
    for movie in movies:
        info=json.loads(subprocess.check_output([args.ffprobe,"-v","error","-show_format","-show_streams","-of","json",str(movie)],text=True,encoding="utf-8"))
        stream_info=[stream for stream in info["streams"] if stream["codec_type"]=="video"]
        assert len(stream_info)==1 and (stream_info[0]["width"],stream_info[0]["height"])==(args.width,args.height), movie
        assert Fraction(stream_info[0]["r_frame_rate"])==Fraction(str(args.fps)), movie
        duration=float(info["format"]["duration"])
        sheet=Image.new("RGB",(args.width*3,args.height+30),"#111111")
        draw=ImageDraw.Draw(sheet)
        with av.open(str(movie)) as container:
            stream=container.streams.video[0]
            for index,fraction in enumerate((.1,.5,.9)):
                target=duration*fraction
                container.seek(int(target/stream.time_base),stream=stream)
                selected=False
                for frame in container.decode(stream):
                    if frame.time >= target:
                        sheet.paste(frame.to_image().convert("RGB"),(args.width*index,30))
                        selected=True
                        break
                assert selected, (movie, target, "No decoded preview frame")
                draw.text((args.width*index+8,8),f"{movie.stem} | {target:.1f}s",fill="white")
        output=args.output / (movie.stem+".jpg")
        sheet.save(output,quality=90)
        print("[DEBUG]",output)


if __name__ == "__main__":
    main()
