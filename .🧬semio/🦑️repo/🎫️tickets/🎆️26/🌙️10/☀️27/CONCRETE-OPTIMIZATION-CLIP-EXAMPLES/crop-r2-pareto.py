"""✂️ Cut magnified crops of the Pareto keyframes where the layout reports and the contact sheet suspect collisions."""

from pathlib import Path

from PIL import Image

FRAMES = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/frames/Beat5_ParetoFront")
OUT = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/r2/crops")
CROPS = (
    ("en-approximate", (600, 140, 800, 210)),
    ("en-walk_2", (100, 150, 330, 400)),
    ("en-third_2", (600, 130, 800, 220)),
    ("de-knee_3", (900, 420, 1280, 600)),
    ("en-rule_2", (860, 520, 1280, 570)),
    ("en-objectives_2", (60, 330, 340, 400)),
    ("en-dominated_2", (120, 260, 340, 340)),
    ("en-approximate", (120, 230, 420, 270)),
    ("en-front_2", (90, 185, 340, 265)),
    ("en-walk", (600, 150, 940, 230)),
    ("de-walk_2", (110, 600, 1170, 690)),
    ("en-decision", (560, 130, 720, 520)),
)


def main():
    """🖼️ Write every crop at twice its size next to the keyframes' generated folder."""
    OUT.mkdir(parents=True, exist_ok=True)
    for name, box in CROPS:
        image = Image.open(FRAMES / f"Beat5_ParetoFront-{name}.png").convert("RGB")
        cropped = image.crop(box)
        cropped = cropped.resize((cropped.width * 2, cropped.height * 2), Image.LANCZOS)
        target = OUT / f"{name}-{box[0]}-{box[1]}.png"
        cropped.save(target)
        print(target)


if __name__ == "__main__":
    main()
