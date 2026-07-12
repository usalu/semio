import numpy as np, math, random

TOLERANCE = 0.01

def normalize(v):
    v = np.array(v, dtype=float); n = np.linalg.norm(v)
    return v/n if n>0 else v
def cross(a,b): return np.cross(a,b)
def dot(a,b): return np.dot(a,b)
def deg_to_rad(d): return d*math.pi/180.0

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

# d3's own quaternion helpers
def quat_rotate_vector(quat, vector):
    x,y,z,w = quat
    vx,vy,vz = vector
    ix = w*vx + y*vz - z*vy
    iy = w*vy + z*vx - x*vz
    iz = w*vz + x*vy - y*vx
    iw = -x*vx - y*vy - z*vz
    return np.array([ix*w + iw*-x + iy*-z - iz*-y, iy*w + iw*-y + iz*-x - ix*-z, iz*w + iw*-z + ix*-y - iy*-x])

# ---- PHYSICAL TEST ----
# Parent piece at origin=(5,0,0) with a known rotation (45 deg about Z)
parent_origin = np.array([5.0, 0.0, 0.0])
half = math.radians(45)/2
parent_quat = [0,0,math.sin(half),math.cos(half)]  # 45deg about Z, d3 [x,y,z,w] format

attracting_vortex_local_pos = np.array([1.0, 0.0, 0.0])
attracting_vortex_local_dir = np.array([0.0, 1.0, 0.0])

# attracted vortex (in child's OWN local frame) at local origin, direction -Y (so reverse_child = +Y = parent_dir after world... 
# but parent_dir passed to compute_child_plane is in PARENT's LOCAL frame (attracting_vortex_local_dir), not world!
attracted_vortex_local_pos = np.array([0.0, 0.0, 0.0])
attracted_vortex_local_dir = np.array([0.0, -1.0, 0.0])

gap = 2.0

# Candidate A: build parent_matrix using quat_rotate_vector for x_axis/y_axis (d3 convention), fed into plane_input_to_matrix
x_axis_A = quat_rotate_vector(parent_quat, [1,0,0])
y_axis_A = quat_rotate_vector(parent_quat, [0,1,0])
parent_matrix_A = plane_input_to_matrix(parent_origin, x_axis_A, y_axis_A)

result_origin, result_x, result_y = compute_child_plane(
    parent_matrix_A, attracting_vortex_local_pos, attracting_vortex_local_dir,
    attracted_vortex_local_pos, attracted_vortex_local_dir,
    gap, 0.0, 0.0, 0.0, 0.0, 0.0
)
result_z = cross(result_x, result_y)

print("=== Candidate A: parent_matrix from quat_rotate_vector-based x/y axis ===")
print("result origin (attracted object world origin):", result_origin)

# PHYSICAL EXPECTATION using d3's own trusted convention:
attracting_vortex_world_pos = parent_origin + quat_rotate_vector(parent_quat, attracting_vortex_local_pos)
attracting_vortex_world_dir = normalize(quat_rotate_vector(parent_quat, attracting_vortex_local_dir))
expected_attracted_vortex_world_pos = attracting_vortex_world_pos + gap * attracting_vortex_world_dir
# attracted vortex is AT child's own local origin (0,0,0) so attracted object's world origin == attracted vortex world pos
print("expected attracted-object world origin:", expected_attracted_vortex_world_pos)
print("MATCH:", np.allclose(result_origin, expected_attracted_vortex_world_pos, atol=1e-6))

# expected direction: attracted vortex world direction should be OPPOSITE of attracting vortex world direction (facing)
# attracted vortex local dir = (0,-1,0); we need the CHILD's world orientation quaternion to check this.
# result_x/result_y are world x_axis/y_axis of the child's plane -- but we need the actual orientation quaternion
# to rotate attracted_vortex_local_dir. Since we don't have child quat yet directly, reconstruct rotation matrix
# columns [result_x | result_y | result_z] and apply it in the SAME convention as compose (columns as image of e_x,e_y,e_z)
child_rot_matrix = np.array([result_x, result_y, result_z]).T  # columns = result_x, result_y, result_z
attracted_vortex_world_dir_candidate = child_rot_matrix @ attracted_vortex_local_dir
print("attracted vortex world dir (via column-matrix @ local_dir):", attracted_vortex_world_dir_candidate)
print("expected (opposite of attracting world dir):", -attracting_vortex_world_dir)
print("MATCH:", np.allclose(attracted_vortex_world_dir_candidate, -attracting_vortex_world_dir, atol=1e-6))
