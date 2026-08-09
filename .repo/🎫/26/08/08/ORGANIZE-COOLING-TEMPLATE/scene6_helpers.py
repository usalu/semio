def _node(text, color, width=3.1, height=0.58, font_size=BODY_FONT_SIZE):
    """🔲 Rounded taxonomy node with centered caption."""
    box = RoundedRectangle(
        width=width, height=height, corner_radius=0.09,
        color=color, stroke_width=2.2,
        fill_color=color, fill_opacity=0.12,
    )
    label = Text(text, font_size=font_size, color=color).move_to(box)
    return VGroup(box, label)


def _elbow(start, end, color, stroke_width=2.0):
    """🪝 Two-segment connector that drops vertically then runs horizontally."""
    knee = np.array([start[0], end[1], 0.0])
    line = VMobject(color=color, stroke_width=stroke_width)
    line.set_points_as_corners([start, knee, end])
    line.set_stroke(opacity=0.55)
    return line


def _room(center, width, height, color=P_WHITE):
    """🏠 Line-art room shell with a teal floor slab."""
    shell = Rectangle(
        width=width, height=height,
        color=color, stroke_width=3.0, fill_opacity=0,
    ).move_to(center)
    floor = Line(shell.get_corner(DL), shell.get_corner(DR), color=P_TEAL, stroke_width=4)
    return VGroup(shell, floor)


def _fan(pos, color, radius=0.26):
    """🌀 Compact fan glyph — hub plus three blades."""
    ring = Circle(radius=radius, color=color, stroke_width=2.4)
    blades = VGroup(*[
        Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.4).rotate(
            a, about_point=ORIGIN
        )
        for a in (0.0, TAU / 3, 2 * TAU / 3)
    ])
    return VGroup(ring, blades).move_to(pos)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Minimal seated-occupant glyph used as an internal heat source."""
    head = Circle(radius=0.12, color=color, stroke_width=2.2)
    body = RoundedRectangle(
        width=0.32, height=0.44, corner_radius=0.1,
        color=color, stroke_width=2.2,
    ).next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _plume(base, height, color, width=0.5, layers=4):
    """♨️ Rising convective plume drawn as stacked, widening arcs."""
    plume = VGroup()
    for i in range(layers):
        f = (i + 1) / layers
        arc = Arc(
            radius=width * f, start_angle=PI * 0.15, angle=PI * 0.7,
            color=color, stroke_width=2.2,
        )
        arc.set_stroke(opacity=0.75 - 0.13 * i)
        arc.move_to(base + UP * (height * f))
        plume.add(arc)
    return plume


# region Airflow
def _smooth_path(points):
    """〰️ Smooth polyline path for airflow particles.

    ``TipableVMobject`` rather than a bare ``VMobject`` so ``_guides`` can put an
    arrowhead on a copy of the same curve.
    """
    path = TipableVMobject()
    path.set_points_smoothly([np.array(p, dtype=float) for p in points])
    return path


def _guides(paths, color, opacity=0.25, width=2.0, tips=True):
    """🧭 Faint streamlines so the route of the air stays readable between particles."""
    lines = VGroup()
    for path in paths:
        guide = path.copy().set_stroke(color=color, width=width, opacity=opacity)
        guide.set_fill(opacity=0)
        if tips:
            guide.add_tip(tip_length=0.17, tip_width=0.14)
            guide.tip.set_fill(color=color, opacity=opacity + 0.3)
            guide.tip.set_stroke(width=0)
        lines.add(guide)
    return lines


def _flow(
    scene, paths, color, run_time=3.2, waves=3, radius=0.075,
    cycles=2.0, color_end=None, extra=None,
):
    """💨 Continuous particle stream — several waves per path, looping and fading at both ends.

    ``color_end`` lets a particle change colour along its route, which is how the
    beats show air picking up or giving off heat while it crosses a room.
    """
    dots = VGroup()
    meta = []
    for path in paths:
        for w in range(waves):
            dot = Dot(radius=radius, color=color, stroke_width=0)
            dot.set_fill(color, opacity=0.0)
            dot.move_to(path.point_from_proportion(0.0))
            dots.add(dot)
            meta.append((path, w / waves))

    start_c = ManimColor(color)
    end_c = ManimColor(color_end) if color_end else start_c

    def update(group, alpha):
        for dot, (path, offset) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            dot.move_to(path.point_from_proportion(t))
            fade = min(1.0, t / 0.10, (1.0 - t) / 0.10)
            dot.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, fade))

    scene.add(dots)
    anims = [UpdateFromAlphaFunc(dots, update)]
    if extra:
        anims.extend(extra)
    scene.play(*anims, run_time=run_time, rate_func=linear)
    # play() registers each dot individually, so the group handle alone leaves them on screen.
    scene.remove(dots, *dots)


def _pad(scene, budget: float, used: float) -> None:
    """⏱️ Hold the frame so a visual phase fills its narration budget."""
    rem = float(budget) - float(used)
    if rem > 0.05:
        scene.wait(rem)
# endregion


