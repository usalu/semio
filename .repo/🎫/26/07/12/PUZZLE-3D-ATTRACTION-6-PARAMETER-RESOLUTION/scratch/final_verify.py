import numpy as np
import random
import math

TOLERANCE = 0.01

def normalize(v):
    v = np.array(v, dtype=float)
    n = np.linalg.norm(v)
    return v/n if n > 0 else v

def cross(a, b): return np.cross(a, b)
def dot(a, b): return np.dot(a, b)
def deg_to_rad(d): return d*math.pi/180.0

# ---- compose ground truth (verbatim transcription) ----

def plane_input_to_matrix(origin, x_axis, y_axis):
    x=x_axis; y=y_axis; z=cross(x,y)
    return np.array([[x[0],y[0],z[0],origin[0]],[x[1],y[1],z[1],origin[1]],[x[2],y[2],z[2],origin[2]],[0,0,0,1]],dtype=float)
def matrix_to_plane(m):
    return [m[0][3],m[1][3],m[2][3]], [m[0][0],m[1][0],m[2][0]], [m[0][1],m[1][1],m[2][1]]
def mul_mat(a,b):
    af=a.flatten(); bf=b.flatten(); out=np.zeros(16)
    for col in range(4):
        for row in range(4):
            out[col*4+row]=af[row]*bf[col*4]+af[4+row]*bf[col*4+1]+af[8+row]*bf[col*4+2]+af[12+row]*bf[col*4+3]
    return out.reshape(4,4)
def translation(x,y,z):
    return np.array([[1,0,0,x],[0,1,0,y],[0,0,1,z],[0,0,0,1]],dtype=float)
def rotation_axis(axis,angle):
    x,y,z=axis; c=math.cos(angle); s=math.sin(angle); t=1.0-c
    flat=[t*x*x+c,t*x*y+s*z,t*x*z-s*y,0.0, t*x*y-s*z,t*y*y+c,t*y*z+s*x,0.0, t*x*z+s*y,t*y*z-s*x,t*z*z+c,0.0, 0,0,0,1]
    return np.array(flat).reshape(4,4)
def apply_mat_vec3(m,v):
    mf=m.flatten()
    return np.array([mf[0]*v[0]+mf[4]*v[1]+mf[8]*v[2], mf[1]*v[0]+mf[5]*v[1]+mf[9]*v[2], mf[2]*v[0]+mf[6]*v[1]+mf[10]*v[2]])
def quaternion_from_unit_vectors(f,t):
    f=np.array(f,dtype=float); t=np.array(t,dtype=float)
    r = dot(f,t)+1.0
    if r < 0.000001:
        if abs(f[0])>abs(f[2]): quat=np.array([-f[1],f[0],0.0,0.0])
        else: quat=np.array([0.0,-f[2],f[1],0.0])
    else:
        c=cross(f,t); quat=np.array([c[0],c[1],c[2],r])
    return quat/np.linalg.norm(quat)
def quaternion_to_matrix(q):
    x,y,z,w=q
    x2,y2,z2=x+x,y+y,z+z
    xx,xy,xz=x*x2,x*y2,x*z2
    yy,yz,zz=y*y2,y*z2,z*z2
    wx,wy,wz=w*x2,w*y2,w*z2
    flat=[1.0-(yy+zz),xy+wz,xz-wy,0.0, xy-wz,1.0-(xx+zz),yz+wx,0.0, xz+wy,yz-wx,1.0-(xx+yy),0.0, 0,0,0,1]
    return np.array(flat).reshape(4,4)

def compute_child_plane(parent_matrix, parent_point, parent_direction, child_point, child_direction,
                         gap, shift, rise, rotation_deg, turn_deg, tilt_deg):
    parent_dir = normalize(parent_direction); child_dir = normalize(child_direction)
    rotation_rad=deg_to_rad(rotation_deg); turn_rad=deg_to_rad(turn_deg); tilt_rad=deg_to_rad(tilt_deg)
    reverse_child = -child_dir
    cross_vec = cross(parent_dir, reverse_child)
    cross_len = np.linalg.norm(cross_vec)
    if cross_len < TOLERANCE:
        if abs(parent_dir[2]) < TOLERANCE:
            align_quat = quaternion_from_unit_vectors([0.0,1.0,0.0],[0.0,0.0,-1.0])
        else:
            axis = normalize(cross([0.0,0.0,1.0], parent_dir)); half=math.pi/2.0
            align_quat = np.array([axis[0]*math.sin(half),axis[1]*math.sin(half),axis[2]*math.sin(half),math.cos(half)])
    else:
        align_quat = quaternion_from_unit_vectors(reverse_child, parent_dir)
    direction_t = quaternion_to_matrix(align_quat)
    y_axis=np.array([0.0,1.0,0.0])
    parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors(y_axis, parent_dir))
    gap_direction = apply_mat_vec3(parent_rotation_t,[0.0,1.0,0.0])
    shift_direction = apply_mat_vec3(parent_rotation_t,[1.0,0.0,0.0])
    raise_direction = apply_mat_vec3(parent_rotation_t,[0.0,0.0,1.0])
    turn_axis = apply_mat_vec3(parent_rotation_t,[0.0,0.0,1.0])
    tilt_axis = apply_mat_vec3(parent_rotation_t,[1.0,0.0,0.0])
    orientation_t = direction_t
    rotate_t = rotation_axis(parent_dir, -rotation_rad)
    orientation_t = mul_mat(rotate_t, orientation_t)
    turn_axis = apply_mat_vec3(rotate_t, turn_axis)
    tilt_axis = apply_mat_vec3(rotate_t, tilt_axis)
    orientation_t = mul_mat(rotation_axis(turn_axis,turn_rad), orientation_t)
    orientation_t = mul_mat(rotation_axis(tilt_axis,tilt_rad), orientation_t)
    center_child_t = translation(-child_point[0],-child_point[1],-child_point[2])
    transform = mul_mat(orientation_t, center_child_t)
    gap_t = translation(gap_direction[0]*gap, gap_direction[1]*gap, gap_direction[2]*gap)
    shift_t = translation(shift_direction[0]*shift, shift_direction[1]*shift, shift_direction[2]*shift)
    raise_t = translation(raise_direction[0]*rise, raise_direction[1]*rise, raise_direction[2]*rise)
    transform = mul_mat(mul_mat(raise_t, mul_mat(shift_t, gap_t)), transform)
    transform = mul_mat(translation(parent_point[0],parent_point[1],parent_point[2]), transform)
    result = mul_mat(parent_matrix, transform)
    return matrix_to_plane(result)

# ---- d3's own quaternion helpers ----

def quat_mul(a, b):
    return np.array([
        a[3]*b[0] + a[0]*b[3] + a[1]*b[2] - a[2]*b[1],
        a[3]*b[1] - a[0]*b[2] + a[1]*b[3] + a[2]*b[0],
        a[3]*b[2] + a[0]*b[1] - a[1]*b[0] + a[2]*b[3],
        a[3]*b[3] - a[0]*b[0] - a[1]*b[1] - a[2]*b[2],
    ])
def quat_rotate_vector(quat, vector):
    x,y,z,w = quat
    vx,vy,vz = vector
    ix = w*vx + y*vz - z*vy
    iy = w*vy + z*vx - x*vz
    iz = w*vz + x*vy - y*vx
    iw = -x*vx - y*vy - z*vz
    return np.array([ix*w + iw*-x + iy*-z - iz*-y, iy*w + iw*-y + iz*-x - ix*-z, iz*w + iw*-z + ix*-y - iy*-x])
def quat_from_axis_angle(axis, angle):
    ax,ay,az = axis
    length = math.sqrt(ax*ax+ay*ay+az*az)
    if length < 1e-8:
        return np.array([0.0,0.0,0.0,1.0])
    half = angle*0.5
    s = math.sin(half)
    return np.array([ax/length*s, ay/length*s, az/length*s, math.cos(half)])
def quat_conj(q):
    return np.array([-q[0],-q[1],-q[2],q[3]])
def quat_normalize(q):
    n = math.sqrt(sum(c*c for c in q))
    return np.array([c/n for c in q])

# ---- FINAL, validated quaternion-only reformulation ----

def compute_attraction_child_pose(t_A, q_A, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg):
    parent_dir = normalize(d_a)
    child_dir = normalize(d_b)
    reverse_child = -child_dir
    cross_vec = cross(parent_dir, reverse_child)
    cross_len = np.linalg.norm(cross_vec)
    if cross_len < TOLERANCE:
        if abs(parent_dir[2]) < TOLERANCE:
            align_q = quaternion_from_unit_vectors([0.0,1.0,0.0],[0.0,0.0,-1.0])
        else:
            axis = normalize(cross([0.0,0.0,1.0], parent_dir)); half = math.pi/2.0
            align_q = np.array([axis[0]*math.sin(half),axis[1]*math.sin(half),axis[2]*math.sin(half),math.cos(half)])
    else:
        align_q = quaternion_from_unit_vectors(reverse_child, parent_dir)

    pq = quaternion_from_unit_vectors([0.0,1.0,0.0], parent_dir)
    gap_dir = quat_rotate_vector(pq, [0.0,1.0,0.0])
    shift_dir = quat_rotate_vector(pq, [1.0,0.0,0.0])
    raise_dir = quat_rotate_vector(pq, [0.0,0.0,1.0])

    rotate_q = quat_from_axis_angle(parent_dir, -deg_to_rad(rotation_deg))
    turn_axis = quat_rotate_vector(rotate_q, raise_dir)
    tilt_axis = quat_rotate_vector(rotate_q, shift_dir)
    turn_q = quat_from_axis_angle(turn_axis, deg_to_rad(turn_deg))
    tilt_q = quat_from_axis_angle(tilt_axis, deg_to_rad(tilt_deg))

    Q_o = quat_conj(align_q)
    Q_o = quat_mul(Q_o, quat_conj(rotate_q))
    Q_o = quat_mul(Q_o, quat_conj(turn_q))
    Q_o = quat_mul(Q_o, quat_conj(tilt_q))
    Q_o = quat_normalize(Q_o)

    offset = np.array(t_A) + np.array(p_a) + gap*gap_dir + shift*shift_dir + rise*raise_dir
    t_B = quat_rotate_vector(Q_o, offset) - np.array(p_b)
    q_B = quat_normalize(quat_mul(Q_o, q_A))
    return t_B, q_B


def frame_from_quat_candidate_a(q, origin):
    x_axis = quat_rotate_vector(q, [1.0,0.0,0.0])
    y_axis = quat_rotate_vector(q, [0.0,1.0,0.0])
    return plane_input_to_matrix(origin, x_axis, y_axis)


random.seed(123)
worst_pos_err = 0.0
worst_orient_err = 0.0
n_trials = 2000
mismatches = 0
for trial in range(n_trials):
    axis_a = normalize([random.uniform(-1,1) for _ in range(3)])
    half = random.uniform(-math.pi, math.pi)/2
    q_A = quat_normalize(quat_from_axis_angle(axis_a, random.uniform(-math.pi, math.pi)))
    t_A = [random.uniform(-5,5) for _ in range(3)]
    p_a = [random.uniform(-2,2) for _ in range(3)]
    p_b = [random.uniform(-2,2) for _ in range(3)]

    mode = trial % 6
    if mode == 0:
        d_a = [0.0, 1.0, 0.0]; d_b = [0.0, -1.0, 0.0]
    elif mode == 1:
        d_a = [0.0, 0.0, 1.0]; d_b = [0.0, 0.0, 1.0]
    elif mode == 2:
        d_a = [1.0, 0.0, 0.0]; d_b = [1.0, 0.0, 0.0]
    else:
        d_a = normalize([random.uniform(-1,1) for _ in range(3)])
        d_b = normalize([random.uniform(-1,1) for _ in range(3)])

    gap = random.uniform(-3,3)
    shift = random.uniform(-3,3)
    rise = random.uniform(-3,3)
    rotation_deg = random.uniform(-180,180)
    turn_deg = random.uniform(-180,180)
    tilt_deg = random.uniform(-180,180)

    parent_matrix = frame_from_quat_candidate_a(q_A, t_A)
    origin_mat, x_axis_mat, y_axis_mat = compute_child_plane(
        parent_matrix, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg
    )

    t_B, q_B = compute_attraction_child_pose(t_A, q_A, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg)

    pos_err = np.linalg.norm(np.array(origin_mat) - t_B)
    worst_pos_err = max(worst_pos_err, pos_err)

    x_axis_q = quat_rotate_vector(q_B, [1.0,0.0,0.0])
    y_axis_q = quat_rotate_vector(q_B, [0.0,1.0,0.0])
    orient_err = max(np.linalg.norm(np.array(x_axis_mat)-x_axis_q), np.linalg.norm(np.array(y_axis_mat)-y_axis_q))
    worst_orient_err = max(worst_orient_err, orient_err)

    if pos_err > 1e-6 or orient_err > 1e-6:
        mismatches += 1
        if mismatches <= 5:
            print(f"MISMATCH trial {trial} mode={mode}: pos_err={pos_err:.3e} orient_err={orient_err:.3e}")

print(f"\nRan {n_trials} random trials.")
print(f"worst position error: {worst_pos_err:.3e}")
print(f"worst orientation error: {worst_orient_err:.3e}")
print(f"total mismatches (>1e-6): {mismatches}")
if worst_pos_err < 1e-6 and worst_orient_err < 1e-6:
    print("ALL PASS: quaternion-only reformulation exactly matches compose's compute_child_plane.")
else:
    print("FAIL: still diverges.")
