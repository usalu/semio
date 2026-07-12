import numpy as np
import random
import math
import sys
sys.path.insert(0, '.')
from final_verify import (normalize, cross, dot, deg_to_rad, quat_mul, quat_rotate_vector,
                           quat_from_axis_angle, quat_conj, quat_normalize,
                           quaternion_from_unit_vectors, compute_attraction_child_pose, TOLERANCE)

def rad_to_deg(r): return r*180.0/math.pi

def standard_matrix(q):
    # columns = quat_rotate_vector(q, e_i); Rmat @ v == quat_rotate_vector(q, v) for all v
    return np.array([quat_rotate_vector(q,[1,0,0]), quat_rotate_vector(q,[0,1,0]), quat_rotate_vector(q,[0,0,1])]).T

def compute_align_q(d_a, d_b):
    parent_dir = normalize(d_a)
    child_dir = normalize(d_b)
    reverse_child = -child_dir
    cross_vec = cross(parent_dir, reverse_child)
    cross_len = np.linalg.norm(cross_vec)
    if cross_len < TOLERANCE:
        if abs(parent_dir[2]) < TOLERANCE:
            return quaternion_from_unit_vectors([0.0,1.0,0.0],[0.0,0.0,-1.0])
        else:
            axis = normalize(cross([0.0,0.0,1.0], parent_dir)); half = math.pi/2.0
            return np.array([axis[0]*math.sin(half),axis[1]*math.sin(half),axis[2]*math.sin(half),math.cos(half)])
    else:
        return quaternion_from_unit_vectors(reverse_child, parent_dir)

def derive_attraction_params(t_A, q_A, p_a, d_a, p_b, d_b, t_B, q_B):
    parent_dir = normalize(d_a)
    align_q = compute_align_q(d_a, d_b)
    pq = quaternion_from_unit_vectors([0.0,1.0,0.0], parent_dir)
    gap_dir = quat_rotate_vector(pq, [0.0,1.0,0.0])
    shift_dir = quat_rotate_vector(pq, [1.0,0.0,0.0])
    raise_dir = quat_rotate_vector(pq, [0.0,0.0,1.0])

    # Q_o such that q_B = quat_mul(Q_o, q_A)  =>  Q_o = quat_mul(q_B, conj(q_A))
    Q_o = quat_normalize(quat_mul(q_B, quat_conj(q_A)))

    # translation: t_B = quat_rotate_vector(Q_o, offset) - p_b  =>  offset = quat_rotate_vector(conj(Q_o), t_B+p_b)
    offset = quat_rotate_vector(quat_conj(Q_o), np.array(t_B) + np.array(p_b))
    diff = offset - np.array(t_A) - np.array(p_a)
    gap = dot(diff, gap_dir)
    shift = dot(diff, shift_dir)
    rise = dot(diff, raise_dir)

    # rotation/turn/tilt: R = quat_mul(align_q, Q_o); M = conj(pq)*R*pq = RotZ(-turn)*RotX(-tilt)*RotY(rotation)
    R = quat_mul(align_q, Q_o)
    M = quat_mul(quat_mul(quat_conj(pq), R), pq)
    Rmat = standard_matrix(M)

    cos_tilt_sq_indicator = Rmat[2][1]  # = -sin(tilt_rad)... using derivation: Rmat[2][1] = sin(b) with b=-tilt_rad => Rmat[2][1] = -sin(tilt_rad)
    if abs(abs(Rmat[2][1]) - 1.0) < 1e-6:
        # gimbal lock: tilt = +-90deg
        tilt_rad = -math.asin(max(-1.0, min(1.0, Rmat[2][1])))
        turn_rad = 0.0
        # with turn=0: Rmat[0][0] = cos(rotation), Rmat[1][0] = sin(b)*sin(... ) -- fall back using Rmat[0][0], Rmat[0][2]
        rotation_rad = math.atan2(Rmat[1][0], Rmat[0][0])
    else:
        tilt_rad = -math.asin(max(-1.0, min(1.0, Rmat[2][1])))
        rotation_rad = math.atan2(-Rmat[2][0], Rmat[2][2])
        turn_rad = math.atan2(Rmat[0][1], Rmat[1][1])

    return gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad)


random.seed(99)
n_trials = 1000
worst_roundtrip_pos = 0.0
worst_roundtrip_orient = 0.0
mismatches = 0
for trial in range(n_trials):
    axis_a = normalize([random.uniform(-1,1) for _ in range(3)])
    q_A = quat_normalize(quat_from_axis_angle(axis_a, random.uniform(-math.pi, math.pi)))
    t_A = [random.uniform(-5,5) for _ in range(3)]
    p_a = [random.uniform(-2,2) for _ in range(3)]
    p_b = [random.uniform(-2,2) for _ in range(3)]

    mode = trial % 5
    if mode == 0:
        d_a = [0.0, 1.0, 0.0]; d_b = [0.0, -1.0, 0.0]
    elif mode == 1:
        d_a = [0.0, 0.0, 1.0]; d_b = [0.0, 0.0, 1.0]
    else:
        d_a = normalize([random.uniform(-1,1) for _ in range(3)])
        d_b = normalize([random.uniform(-1,1) for _ in range(3)])

    gap = random.uniform(-3,3)
    shift = random.uniform(-3,3)
    rise = random.uniform(-3,3)
    rotation_deg = random.uniform(-179,179)
    turn_deg = random.uniform(-179,179)
    tilt_deg = random.uniform(-89,89)   # avoid gimbal lock region for the primary round-trip check

    t_B, q_B = compute_attraction_child_pose(t_A, q_A, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg)

    derived = derive_attraction_params(t_A, q_A, p_a, d_a, p_b, d_b, t_B, q_B)
    d_gap, d_shift, d_rise, d_rotation, d_turn, d_tilt = derived

    # re-apply forward with DERIVED params; must reproduce the SAME t_B, q_B (the actual invariant we need)
    t_B2, q_B2 = compute_attraction_child_pose(t_A, q_A, p_a, d_a, p_b, d_b, d_gap, d_shift, d_rise, d_rotation, d_turn, d_tilt)

    pos_err = np.linalg.norm(np.array(t_B) - t_B2)
    # orientation error via test vectors (handles quaternion sign ambiguity)
    v1 = quat_rotate_vector(q_B, [1,0,0]); v2 = quat_rotate_vector(q_B2, [1,0,0])
    v3 = quat_rotate_vector(q_B, [0,1,0]); v4 = quat_rotate_vector(q_B2, [0,1,0])
    orient_err = max(np.linalg.norm(v1-v2), np.linalg.norm(v3-v4))

    worst_roundtrip_pos = max(worst_roundtrip_pos, pos_err)
    worst_roundtrip_orient = max(worst_roundtrip_orient, orient_err)
    if pos_err > 1e-5 or orient_err > 1e-5:
        mismatches += 1
        if mismatches <= 5:
            print(f"ROUNDTRIP MISMATCH trial {trial} mode={mode}: pos_err={pos_err:.3e} orient_err={orient_err:.3e}")
            print(f"  original params: gap={gap:.3f} shift={shift:.3f} rise={rise:.3f} rot={rotation_deg:.3f} turn={turn_deg:.3f} tilt={tilt_deg:.3f}")
            print(f"  derived params:  gap={d_gap:.3f} shift={d_shift:.3f} rise={d_rise:.3f} rot={d_rotation:.3f} turn={d_turn:.3f} tilt={d_tilt:.3f}")

print(f"\nRan {n_trials} forward->derive->forward round-trip trials.")
print(f"worst roundtrip position error: {worst_roundtrip_pos:.3e}")
print(f"worst roundtrip orientation error: {worst_roundtrip_orient:.3e}")
print(f"mismatches: {mismatches}")
if worst_roundtrip_pos < 1e-5 and worst_roundtrip_orient < 1e-5:
    print("ALL PASS: derive_attraction_params is a correct inverse (round-trips exactly, idempotent).")
else:
    print("FAIL.")
