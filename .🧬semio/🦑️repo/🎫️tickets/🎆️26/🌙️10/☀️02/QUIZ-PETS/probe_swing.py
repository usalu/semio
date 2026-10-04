#!/usr/bin/env python3
"""🔬️ Throw-away numbers behind the constants and tolerances of work package A3 (swing, parachute, arctangent).

Run from the repository root: ``.venv/Scripts/python.exe <this file> [section …]``. Nothing is written; every
section prints what it measured. The arithmetic is the one the module and the oracles use.
"""

import math
import sys

import numpy
import scipy.integrate

sys.stdout.reconfigure(encoding="utf-8")
TAU = 6.283185307179586
A = (0.9998660, -0.3302995, 0.1801410, -0.0851330, 0.0208351)


def atan_unit(z):
    z2 = z * z
    return z * (A[0] + z2 * (A[1] + z2 * (A[2] + z2 * (A[3] + z2 * A[4]))))


def atan_turns(y, x):
    ay, ax = abs(y), abs(x)
    if ay == 0 and ax == 0:
        return 0.0
    t = atan_unit(ay / ax) / TAU if ay <= ax else 0.25 - atan_unit(ax / ay) / TAU
    if x < 0:
        t = 0.5 - t
    if y < 0:
        t = 0 - t
    return 0.5 if t <= -0.5 else t


def fast_neg_exp(x):
    x = x if x > 0 else 0.0
    return 1 / (1 + x * (1 + x * (0.48 + 0.235 * x)))


def swing_step(a0, a1, bob, prev, length, gravity, damping, rope):
    ux = bob[0] + (bob[0] - prev[0]) * damping
    uy = bob[1] + (bob[1] - prev[1]) * damping + gravity / 4096
    rx, ry = bob[0] - a0[0], bob[1] - a0[1]
    wx, wy = ux - a1[0], uy - a1[1]
    rr, wr, ww = rx * rx + ry * ry, wx * rx + wy * ry, wx * wx + wy * wy
    if rope and ww <= length * length:
        return (ux, uy)
    lam = (wr - math.sqrt(max(0.0, wr * wr - rr * (ww - length * length)))) / max(rr, 1e-9)
    return (ux - lam * rx, uy - lam * ry)


def naive_step(a1, bob, prev, length, gravity, damping):
    ux = bob[0] + (bob[0] - prev[0]) * damping
    uy = bob[1] + (bob[1] - prev[1]) * damping + gravity / 4096
    dx, dy = ux - a1[0], uy - a1[1]
    d = math.sqrt(dx * dx + dy * dy)
    return (a1[0] + dx * length / d, a1[1] + dy * length / d)


def pendulum(theta0, omega0, g, length, times, gamma=0.0):
    solution = scipy.integrate.solve_ivp(lambda t, s: [s[1], -(g / length) * math.sin(s[0]) - gamma * s[1]], (min(0.0, times[0]), times[-1]) if times[0] >= 0 else (0.0, times[-1]), [theta0, omega0], method="DOP853", t_eval=times, rtol=1e-12, atol=1e-12)
    return solution.y[0]


def section_atan():
    worst = 0.0
    rng = numpy.random.default_rng(1)
    for _ in range(400000):
        y, x = rng.normal(), rng.normal()
        worst = max(worst, abs(atan_turns(y, x) - math.atan2(y, x) / (2 * math.pi)))
    grid = numpy.linspace(0, 1, 200001)
    unit = max(abs(atan_unit(z) - math.atan(z)) for z in grid)
    print("atanTurns max error (turns) on 400000 gaussian points: %.3e; polynomial on [0,1] max error (rad): %.3e" % (worst, unit))
    print("jump at the diagonal (turns): %.3e" % abs((atan_unit(1.0) / TAU) - (0.25 - atan_unit(1.0) / TAU)))
    mono = all(atan_unit(grid[i + 1]) > atan_unit(grid[i]) for i in range(len(grid) - 1))
    print("polynomial rises on [0,1]: %s" % mono)


def section_exp():
    grid = numpy.linspace(0, 20, 2000001)
    values = 1 / (1 + grid * (1 + grid * (0.48 + 0.235 * grid)))
    error = numpy.abs(values - numpy.exp(-grid))
    print("fastNegExp max abs error %.4e at x = %.4f" % (error.max(), grid[error.argmax()]))
    print("monotone falling: %s" % bool(numpy.all(numpy.diff(values) < 0)))


def run_rod(theta0, g, length, ticks, damping=1.0, start="exact", step="shake", gamma=0.0):
    dt = 1 / 64
    if start == "exact":
        before = pendulum(theta0, 0.0, g, length, [0.0, dt], gamma)[1] if gamma == 0.0 else None
        prev = (length * math.sin(before), length * math.cos(before))
    else:
        prev = (length * math.sin(theta0), length * math.cos(theta0))
    bob = (length * math.sin(theta0), length * math.cos(theta0))
    out = [bob]
    for _ in range(ticks):
        new = swing_step((0, 0), (0, 0), bob, prev, length, g, damping, False) if step == "shake" else naive_step((0, 0), bob, prev, length, g, damping)
        prev, bob = bob, new
        out.append(bob)
    return out


def angles(path):
    return numpy.degrees(numpy.arctan2([p[0] for p in path], [p[1] for p in path]))


def peaks(path, length):
    theta = numpy.unwrap(numpy.radians(angles(path)))
    found = []
    for i in range(1, len(theta) - 1):
        if abs(theta[i]) >= abs(theta[i - 1]) and abs(theta[i]) > abs(theta[i + 1]) and abs(theta[i]) > 1e-3:
            fit = numpy.polyfit([-1, 0, 1], numpy.abs(theta[i - 1 : i + 2]), 2)
            found.append((i, math.degrees(fit[2] - fit[1] * fit[1] / (4 * fit[0]))))
    return found


def section_rod():
    for theta_degrees, g, length in ((45, 1800, 100), (60, 7200, 38.4), (20, 1800, 100), (120, 1800, 100), (30, 900, 43.2), (5, 1800, 154)):
        theta0 = math.radians(theta_degrees)
        ticks = 192
        times = numpy.arange(0, ticks + 1) / 64
        reference = numpy.degrees(pendulum(theta0, 0.0, g, length, times))
        for start in ("exact", "rest"):
            for step in ("shake", "naive"):
                produced = angles(run_rod(theta0, g, length, ticks, start=start, step=step))
                error = numpy.abs(produced - reference)
                print("rod %3d° g=%4d L=%5.1f start=%-5s %-5s: max error over 3 s %.4f° (at tick %d), at ticks 16/32/48/64/96/128/160/192: %s" % (theta_degrees, g, length, start, step, error.max(), error.argmax(), " ".join("%.3f" % error[t] for t in (16, 32, 48, 64, 96, 128, 160, 192))))


def section_drift():
    for theta_degrees, g, length in ((45, 1800, 100), (60, 7200, 38.4), (20, 1800, 100), (120, 1800, 100), (30, 900, 43.2)):
        theta0 = math.radians(theta_degrees)
        for step in ("shake", "naive"):
            path = run_rod(theta0, g, length, 3840, start="exact", step=step)
            found = peaks(path, length)
            amplitudes = [amplitude for _tick, amplitude in found]
            print("drift %3d° g=%4d L=%5.1f %-5s: peaks %d, first %.4f° last %.4f°, largest departure from the start %.4f°" % (theta_degrees, g, length, step, len(found), amplitudes[0], amplitudes[-1], max(abs(a - theta_degrees) for a in amplitudes)))


def spring_step(position, velocity, target, stiffness, damping):
    quickened = velocity + (stiffness * (target - position) - damping * velocity) * 0.015625
    return position + quickened * 0.015625, quickened


def minimum_jerk(distance, duration, t):
    s = min(max(t / duration, 0.0), 1.0)
    return distance * (10 * s**3 - 15 * s**4 + 6 * s**5)


def section_drag():
    for stiffness, damping_spring in ((534, 46), (529, 46)):
        position, velocity = 0.0, 0.0
        trace = []
        for tick in range(40):
            position, velocity = spring_step(position, velocity, 100.0, stiffness, damping_spring)
            trace.append(position)
        half = next(i + 1 for i, p in enumerate(trace) if p >= 50)
        print("follow k=%d c=%d: step response after 1,2,4,8,16 ticks %s; half way at tick %d; max %.4f" % (stiffness, damping_spring, " ".join("%.2f" % trace[i - 1] for i in (1, 2, 4, 8, 16)), half, max(trace)))
        position, velocity = 0.0, 0.0
        for tick in range(400):
            target = 640.0 * (tick + 1) / 64
            position, velocity = spring_step(position, velocity, target, stiffness, damping_spring)
        print("  steady lag behind a pointer at 640 px/s: %.3f px = %.1f ms" % (640.0 * 400 / 64 - position, (640.0 * 400 / 64 - position) / 640 * 1000))
    length = 38.4
    for gscale in (3, 4, 5):
        for distance, duration in ((120, 0.4), (300, 0.5), (600, 0.35), (200, 1.0)):
            ax, vx = 0.0, 0.0
            anchor = (0.0, 0.0)
            bob, prev = (0.0, length), (0.0, length)
            peak, settle = 0.0, 0
            for tick in range(1, 400):
                target = minimum_jerk(distance, duration, tick / 64)
                ax, vx = spring_step(ax, vx, target, 534, 46)
                new_anchor = (ax, 0.0)
                new = swing_step(anchor, new_anchor, bob, prev, length, 1800 * gscale, 0.95, False)
                dy = new[1] - new_anchor[1]
                if dy < 0.5 * length:
                    dx = new[0] - new_anchor[0]
                    side = math.sqrt(length * length - 0.25 * length * length)
                    new = (new_anchor[0] + (-side if dx < 0 else side), new_anchor[1] + 0.5 * length)
                prev, bob, anchor = bob, new, new_anchor
                lean = abs(math.degrees(math.atan2(bob[0] - anchor[0], bob[1] - anchor[1])))
                peak = max(peak, lean)
                if lean >= 3:
                    settle = tick
            print("drag %3d px in %.2f s, held gravity ×%d: peak lean %.1f°, below 3° from %.2f s" % (distance, duration, gscale, peak, (settle + 1) / 64))


def section_chute():
    g, v_open, v_chute, h_min, reflex, factor = 1800.0, 240.0, 600.0, 56.0, 6, 0.9168553557320289
    print("factor exp(-1/(64·0.18)) = %.17g" % math.exp(-1 / (64 * 0.18)))
    for height_body in (48,):
        terminal, flare = 2.0 * height_body, 0.25 * height_body
        for forced in (False, True):
            for drop in (36, 50, 80, 100, 101, 102, 103, 104, 105, 110, 150, 200, 300, 500):
                y, vy, opened, age, tick = 0.0, 0.0, None, 0, 0
                rate = 0.0
                while True:
                    tick += 1
                    remaining = drop - y
                    if opened is None:
                        hit = math.sqrt(vy * vy + 2 * g * remaining)
                        if vy >= v_open and remaining >= h_min and (forced or hit > v_chute):
                            opened = tick
                    if opened is None or tick - opened < reflex:
                        vy = min(vy + g / 64, 900.0)
                        rate = vy
                    else:
                        vy = terminal + (vy - terminal) * factor
                        rate = vy * (0.5 + 0.5 * remaining / flare) if remaining < flare else vy
                    y += rate / 64
                    if y >= drop:
                        break
                print("drop %3d px %s: no chute %.0f, opened at tick %s, lands at tick %d with %.1f px/s (unflared %.1f)" % (drop, "forced" if forced else "rule  ", min(math.sqrt(2 * g * drop), 900), opened, tick, rate, vy))


SECTIONS = {"atan": section_atan, "exp": section_exp, "rod": section_rod, "drift": section_drift, "drag": section_drag, "chute": section_chute}

if __name__ == "__main__":
    for name in sys.argv[1:] or SECTIONS:
        print("== %s" % name)
        SECTIONS[name]()
