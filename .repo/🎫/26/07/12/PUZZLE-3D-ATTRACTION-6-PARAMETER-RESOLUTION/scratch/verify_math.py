import numpy as np
import random
import math

TOLERANCE = 0.01

# ---- transcribed verbatim from compose/client/lib/rs/lib.rs ----

def normalize(v):
    v = np.array(v, dtype=float)
    n = np.linalg.norm(v)
    if n > 0:
        return v / n
    return v

def cross(a, b):
    return np.cross(a, b)

def dot(a, b):
    return np.dot(a, b)

def deg_to_rad(d):
    return d * math.pi / 180.0

def plane_input_to_matrix(origin, x_axis, y_axis):
    x = x_axis
    y = y_axis
    z = cross(x, y)
    return np.array([
        [x[0], y[0], z[0], origin[0]],
        [x[1], y[1], z[1], origin[1]],
        [x[2], y[2], z[2], origin[2]],
        [0,0,0,1]
    ], dtype=float)

def matrix_to_plane(m):
    origin = [m[0][3], m[1][3], m[2][3]]
    x_axis = [m[0][0], m[1][0], m[2][0]]
    y_axis = [m[0][1], m[1][1], m[2][1]]
    return origin, x_axis, y_axis

def mul_mat(a, b):
    # literal transcription of the Rust code (with row-major 16-array flattened from 4x4 'a','b')
    af = a.flatten()
    bf = b.flatten()
    out = np.zeros(16)
    for col in range(4):
        for row in range(4):
            out[col*4+row] = (af[row]*bf[col*4]
                               + af[4+row]*bf[col*4+1]
                               + af[8+row]*bf[col*4+2]
                               + af[12+row]*bf[col*4+3])
    return out.reshape(4,4)

def translation(x,y,z):
    return np.array([
        [1,0,0,x],
        [0,1,0,y],
        [0,0,1,z],
        [0,0,0,1]
    ], dtype=float)

def rotation_axis(axis, angle):
    x,y,z = axis
    c = math.cos(angle)
    s = math.sin(angle)
    t = 1.0 - c
    return np.array([
        [t*x*x+c,    t*x*y-s*z,  t*x*z+s*y,  0.0],
        [t*x*y+s*z,  t*y*y+c,    t*y*z-s*x,  0.0],
        [t*x*z-s*y,  t*y*z+s*x,  t*z*z+c,    0.0],
        [0,0,0,1]
    ], dtype=float)
    # NOTE: transcribed by reading the flat array in row-major order:
    # [t*x*x+c, t*x*y+s*z, t*x*z-s*y, 0,
    #  t*x*y-s*z, t*y*y+c, t*y*z+s*x, 0,
    #  t*x*z+s*y, t*y*z-s*x, t*z*z+c, 0,
    #  0,0,0,1]

def rotation_axis_literal(axis, angle):
    x,y,z = axis
    c = math.cos(angle)
    s = math.sin(angle)
    t = 1.0 - c
    flat = [
        t*x*x+c, t*x*y+s*z, t*x*z-s*y, 0.0,
        t*x*y-s*z, t*y*y+c, t*y*z+s*x, 0.0,
        t*x*z+s*y, t*y*z-s*x, t*z*z+c, 0.0,
        0.0,0.0,0.0,1.0,
    ]
    return np.array(flat).reshape(4,4)

def apply_mat_vec3(m, v):
    # rust: m[0]*v0+m[4]*v1+m[8]*v2, m[1]*v0+m[5]*v1+m[9]*v2, m[2]*v0+m[6]*v1+m[10]*v2
    mf = m.flatten()
    return np.array([
        mf[0]*v[0]+mf[4]*v[1]+mf[8]*v[2],
        mf[1]*v[0]+mf[5]*v[1]+mf[9]*v[2],
        mf[2]*v[0]+mf[6]*v[1]+mf[10]*v[2],
    ])

def quaternion_from_unit_vectors(f, t):
    f = np.array(f,dtype=float); t=np.array(t,dtype=float)
    r = dot(f,t) + 1.0
    if r < 0.000001:
        if abs(f[0]) > abs(f[2]):
            quat = np.array([-f[1], f[0], 0.0, 0.0])
        else:
            quat = np.array([0.0, -f[2], f[1], 0.0])
    else:
        c = cross(f,t)
        quat = np.array([c[0],c[1],c[2],r])
    return quat/np.linalg.norm(quat)

def quaternion_to_matrix(q):
    x,y,z,w = q
    x2,y2,z2 = x+x,y+y,z+z
    xx,xy,xz = x*x2, x*y2, x*z2
    yy,yz,zz = y*y2, y*z2, z*z2
    wx,wy,wz = w*x2, w*y2, w*z2
    flat = [
        1.0-(yy+zz), xy+wz, xz-wy, 0.0,
        xy-wz, 1.0-(xx+zz), yz+wx, 0.0,
        xz+wy, yz-wx, 1.0-(xx+yy), 0.0,
        0.0,0.0,0.0,1.0,
    ]
    return np.array(flat).reshape(4,4)

def compute_child_plane(parent_matrix, parent_point, parent_direction, child_point, child_direction,
                         gap, shift, rise, rotation_deg, turn_deg, tilt_deg):
    parent_dir = normalize(parent_direction)
    child_dir = normalize(child_direction)
    rotation_rad = deg_to_rad(rotation_deg)
    turn_rad = deg_to_rad(turn_deg)
    tilt_rad = deg_to_rad(tilt_deg)
    reverse_child = -child_dir
    cross_vec = cross(parent_dir, reverse_child)
    cross_len = np.linalg.norm(cross_vec)
    if cross_len < TOLERANCE:
        if abs(parent_dir[2]) < TOLERANCE:
            align_quat = quaternion_from_unit_vectors([0.0,1.0,0.0],[0.0,0.0,-1.0])
        else:
            axis = normalize(cross([0.0,0.0,1.0], parent_dir))
            half = math.pi/2.0
            align_quat = np.array([axis[0]*math.sin(half), axis[1]*math.sin(half), axis[2]*math.sin(half), math.cos(half)])
    else:
        align_quat = quaternion_from_unit_vectors(reverse_child, parent_dir)
    direction_t = quaternion_to_matrix(align_quat)
    y_axis = np.array([0.0,1.0,0.0])
    parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors(y_axis, parent_dir))
    gap_direction = apply_mat_vec3(parent_rotation_t, [0.0,1.0,0.0])
    shift_direction = apply_mat_vec3(parent_rotation_t, [1.0,0.0,0.0])
    raise_direction = apply_mat_vec3(parent_rotation_t, [0.0,0.0,1.0])
    turn_axis = apply_mat_vec3(parent_rotation_t, [0.0,0.0,1.0])
    tilt_axis = apply_mat_vec3(parent_rotation_t, [1.0,0.0,0.0])
    orientation_t = direction_t
    rotate_t = rotation_axis_literal(parent_dir, -rotation_rad)
    orientation_t = mul_mat(rotate_t, orientation_t)
    turn_axis = apply_mat_vec3(rotate_t, turn_axis)
    tilt_axis = apply_mat_vec3(rotate_t, tilt_axis)
    orientation_t = mul_mat(rotation_axis_literal(turn_axis, turn_rad), orientation_t)
    orientation_t = mul_mat(rotation_axis_literal(tilt_axis, tilt_rad), orientation_t)
    center_child_t = translation(-child_point[0], -child_point[1], -child_point[2])
    transform = mul_mat(orientation_t, center_child_t)
    gap_t = translation(gap_direction[0]*gap, gap_direction[1]*gap, gap_direction[2]*gap)
    shift_t = translation(shift_direction[0]*shift, shift_direction[1]*shift, shift_direction[2]*shift)
    raise_t = translation(raise_direction[0]*rise, raise_direction[1]*rise, raise_direction[2]*rise)
    transform = mul_mat(mul_mat(raise_t, mul_mat(shift_t, gap_t)), transform)
    transform = mul_mat(translation(parent_point[0], parent_point[1], parent_point[2]), transform)
    result = mul_mat(parent_matrix, transform)
    return matrix_to_plane(result)

# ---- d3's own quaternion helpers (transcribed verbatim) ----

def quat_mul(a, b):
    # a,b = [x,y,z,w]
    return [
        a[3]*b[0] + a[0]*b[3] + a[1]*b[2] - a[2]*b[1],
        a[3]*b[1] - a[0]*b[2] + a[1]*b[3] + a[2]*b[0],
        a[3]*b[2] + a[0]*b[1] - a[1]*b[0] + a[2]*b[3],
        a[3]*b[3] - a[0]*b[0] - a[1]*b[1] - a[2]*b[2],
    ]

def quat_rotate_vector(quat, vector):
    x,y,z,w = quat
    vx,vy,vz = vector
    ix = w*vx + y*vz - z*vy
    iy = w*vy + z*vx - x*vz
    iz = w*vz + x*vy - y*vx
    iw = -x*vx - y*vy - z*vz
    return np.array([
        ix*w + iw*-x + iy*-z - iz*-y,
        iy*w + iw*-y + iz*-x - ix*-z,
        iz*w + iw*-z + ix*-y - iy*-x,
    ])

def quat_from_axis_angle(ax,ay,az,angle):
    length = math.sqrt(ax*ax+ay*ay+az*az)
    if length < 1e-8:
        return [0.0,0.0,0.0,1.0]
    half = angle*0.5
    s = math.sin(half)
    return [ax/length*s, ay/length*s, az/length*s, math.cos(half)]

def quat_conjugate(q):
    return [-q[0],-q[1],-q[2],q[3]]

def quat_normalize(q):
    n = math.sqrt(sum(c*c for c in q))
    return [c/n for c in q]

# STEP 1: sanity check that quat_rotate_vector(q, e) matches apply_mat_vec3(quaternion_to_matrix(q), e)
random.seed(42)
for trial in range(20):
    axis = normalize([random.uniform(-1,1) for _ in range(3)])
    angle = random.uniform(-math.pi, math.pi)
    q = quat_from_axis_angle(axis[0],axis[1],axis[2],angle)
    for e in [[1,0,0],[0,1,0],[0,0,1],[0.3,0.7,-0.2]]:
        a = quat_rotate_vector(q, e)
        b = apply_mat_vec3(quaternion_to_matrix(q), e)
        assert np.allclose(a,b, atol=1e-9), f"MISMATCH quat_rotate_vector vs matrix: {a} vs {b}"
print("STEP 1 PASS: quat_rotate_vector(q,v) == apply_mat_vec3(quaternion_to_matrix(q), v) for all tested q,v")

# STEP 2: sanity check that mul_mat(A,B) applied to a vector matches applying B then A (i.e. mul_mat(A,B)*v == A*(B*v))
# i.e. mul_mat(A,B) represents "apply B first, then A" -- the usual math convention result = A o B
for trial in range(20):
    axis1 = normalize([random.uniform(-1,1) for _ in range(3)])
    axis2 = normalize([random.uniform(-1,1) for _ in range(3)])
    qa = quat_from_axis_angle(*axis1, random.uniform(-math.pi,math.pi))
    qb = quat_from_axis_angle(*axis2, random.uniform(-math.pi,math.pi))
    A = quaternion_to_matrix(qa)
    B = quaternion_to_matrix(qb)
    AB = mul_mat(A,B)
    for e in [[1,0,0],[0,1,0],[0,0,1],[0.3,0.7,-0.2]]:
        lhs = apply_mat_vec3(AB, e)
        rhs = apply_mat_vec3(A, apply_mat_vec3(B, e))
        assert np.allclose(lhs, rhs, atol=1e-9), f"mul_mat convention MISMATCH: {lhs} vs {rhs}"
print("STEP 2 PASS: mul_mat(A,B) applied to v == A applied to (B applied to v)  [i.e. mul_mat(A,B) = A after B, standard composition]")

print("\nAll sanity checks passed. Proceeding to build the quaternion-only reformulation.")

# STEP 3: verify quat_mul(q,r) applied via quat_rotate_vector represents "apply r first, then q"
for trial in range(20):
    axis1 = normalize([random.uniform(-1,1) for _ in range(3)])
    axis2 = normalize([random.uniform(-1,1) for _ in range(3)])
    qa = quat_from_axis_angle(*axis1, random.uniform(-math.pi,math.pi))
    qb = quat_from_axis_angle(*axis2, random.uniform(-math.pi,math.pi))
    combined = quat_mul(qa, qb)
    for e in [[1,0,0],[0,1,0],[0,0,1],[0.3,0.7,-0.2]]:
        lhs = quat_rotate_vector(combined, e)
        rhs = quat_rotate_vector(qa, quat_rotate_vector(qb, e))
        assert np.allclose(lhs, rhs, atol=1e-9), f"quat_mul convention MISMATCH: {lhs} vs {rhs}"
print("STEP 3 PASS: quat_rotate_vector(quat_mul(q,r), v) == quat_rotate_vector(q, quat_rotate_vector(r, v))  [quat_mul(q,r) = q after r]")

# STEP 4: verify rotation_axis_literal(axis, angle) == quaternion_to_matrix(quat_from_axis_angle(axis, angle))
for trial in range(20):
    axis = normalize([random.uniform(-1,1) for _ in range(3)])
    angle = random.uniform(-math.pi, math.pi)
    M1 = rotation_axis_literal(axis, angle)
    q = quat_from_axis_angle(axis[0],axis[1],axis[2],angle)
    M2 = quaternion_to_matrix(q)
    assert np.allclose(M1, M2, atol=1e-9), f"rotation_axis vs quat_from_axis_angle MISMATCH:\n{M1}\nvs\n{M2}"
print("STEP 4 PASS: rotation_axis_literal(axis,angle) == quaternion_to_matrix(quat_from_axis_angle(axis,angle))")


# ---- Full quaternion-only reformulation of compute_child_plane ----

def compute_child_pose_quat(q_A, t_A, p_a, d_a_raw, p_b, d_b_raw, gap, shift, rise, rotation_deg, turn_deg, tilt_deg):
    d_a = normalize(d_a_raw)
    d_b = normalize(d_b_raw)
    rotation_rad = deg_to_rad(rotation_deg)
    turn_rad = deg_to_rad(turn_deg)
    tilt_rad = deg_to_rad(tilt_deg)
    reverse_child = -d_b
    cross_vec = cross(d_a, reverse_child)
    cross_len = np.linalg.norm(cross_vec)
    if cross_len < TOLERANCE:
        if abs(d_a[2]) < TOLERANCE:
            align_q = quaternion_from_unit_vectors([0.0,1.0,0.0],[0.0,0.0,-1.0])
        else:
            axis = normalize(cross([0.0,0.0,1.0], d_a))
            half = math.pi/2.0
            align_q = np.array([axis[0]*math.sin(half), axis[1]*math.sin(half), axis[2]*math.sin(half), math.cos(half)])
    else:
        align_q = quaternion_from_unit_vectors(reverse_child, d_a)

    pq = quaternion_from_unit_vectors([0.0,1.0,0.0], d_a)
    gap_dir = quat_rotate_vector(pq, [0.0,1.0,0.0])
    shift_dir = quat_rotate_vector(pq, [1.0,0.0,0.0])
    rise_dir = quat_rotate_vector(pq, [0.0,0.0,1.0])

    rotate_q = quat_from_axis_angle(d_a[0], d_a[1], d_a[2], -rotation_rad)
    turn_axis = quat_rotate_vector(rotate_q, rise_dir)
    tilt_axis = quat_rotate_vector(rotate_q, shift_dir)

    orientation_q = align_q
    orientation_q = quat_mul(rotate_q, orientation_q)
    turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], turn_rad)
    orientation_q = quat_mul(turn_q, orientation_q)
    tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], tilt_rad)
    orientation_q = quat_mul(tilt_q, orientation_q)
    orientation_q = quat_normalize(orientation_q)

    t_local = np.array(p_a) + gap*gap_dir + shift*shift_dir + rise*rise_dir - quat_rotate_vector(orientation_q, p_b)

    world_orientation = quat_normalize(quat_mul(q_A, orientation_q))
    world_origin = np.array(t_A) + quat_rotate_vector(q_A, t_local)
    return world_origin, world_orientation


def frame_from_quat(q, origin):
    x_axis = quat_rotate_vector(q, [1.0,0.0,0.0])
    y_axis = quat_rotate_vector(q, [0.0,1.0,0.0])
    return plane_input_to_matrix(origin, x_axis, y_axis)


def quat_to_test_vectors(q):
    return [quat_rotate_vector(q, e) for e in [[1,0,0],[0,1,0],[0,0,1]]]


random.seed(7)
worst_pos_err = 0.0
worst_orient_err = 0.0
n_trials = 500
for trial in range(n_trials):
    axis_a = normalize([random.uniform(-1,1) for _ in range(3)])
    q_A = quat_from_axis_angle(*axis_a, random.uniform(-math.pi, math.pi))
    t_A = [random.uniform(-5,5) for _ in range(3)]
    p_a = [random.uniform(-2,2) for _ in range(3)]
    p_b = [random.uniform(-2,2) for _ in range(3)]

    # occasionally force parallel / antiparallel special-case directions to exercise that branch
    mode = trial % 5
    if mode == 0:
        d_a = [0.0, 1.0, 0.0]
        d_b = [0.0, -1.0, 0.0]  # exactly antiparallel-reverse -> aligned already (reverse_child == d_a)
    elif mode == 1:
        d_a = [0.0, 0.0, 1.0]
        d_b = [0.0, 0.0, 1.0]  # reverse_child = -d_b = [0,0,-1], antiparallel to d_a -> triggers special branch
    else:
        d_a = normalize([random.uniform(-1,1) for _ in range(3)])
        d_b = normalize([random.uniform(-1,1) for _ in range(3)])

    gap = random.uniform(-3,3)
    shift = random.uniform(-3,3)
    rise = random.uniform(-3,3)
    rotation_deg = random.uniform(-180,180)
    turn_deg = random.uniform(-180,180)
    tilt_deg = random.uniform(-180,180)

    # matrix-based ground truth: parent_matrix built from q_A/t_A via frame_from_quat (mirrors d3 world_vortex_position convention)
    parent_matrix = frame_from_quat(q_A, t_A)
    origin_mat, x_axis_mat, y_axis_mat = compute_child_plane(
        parent_matrix, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg
    )

    world_origin_q, world_orientation_q = compute_child_pose_quat(
        q_A, t_A, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg
    )

    pos_err = np.linalg.norm(np.array(origin_mat) - world_origin_q)
    worst_pos_err = max(worst_pos_err, pos_err)

    # compare orientation by rotating test vectors (handles quaternion sign ambiguity automatically since q and -q rotate identically)
    x_axis_q = quat_rotate_vector(world_orientation_q, [1.0,0.0,0.0])
    y_axis_q = quat_rotate_vector(world_orientation_q, [0.0,1.0,0.0])
    orient_err = max(np.linalg.norm(np.array(x_axis_mat)-x_axis_q), np.linalg.norm(np.array(y_axis_mat)-y_axis_q))
    worst_orient_err = max(worst_orient_err, orient_err)

    if pos_err > 1e-6 or orient_err > 1e-6:
        print(f"MISMATCH at trial {trial} (mode={mode}): pos_err={pos_err} orient_err={orient_err}")
        print("  d_a", d_a, "d_b", d_b)
        print("  origin_mat", origin_mat, "world_origin_q", world_origin_q)
        print("  x_axis_mat", x_axis_mat, "x_axis_q", x_axis_q)
        print("  y_axis_mat", y_axis_mat, "y_axis_q", y_axis_q)

print(f"\nSTEP 5: ran {n_trials} random trials (incl. parallel/antiparallel special-case directions).")
print(f"  worst position error: {worst_pos_err:.3e}")
print(f"  worst orientation (axis) error: {worst_orient_err:.3e}")
if worst_pos_err < 1e-6 and worst_orient_err < 1e-6:
    print("STEP 5 PASS: quaternion-only reformulation matches compose's matrix-based compute_child_plane exactly.")
else:
    print("STEP 5 FAIL: quaternion reformulation diverges from compose reference math.")

