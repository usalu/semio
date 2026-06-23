#!/usr/bin/env python3
"""Add double-precision matrix helpers for Jacobian computation."""

COMPOSE_CS = r"C:\git\compose\compose\net\Compose\Compose.cs"

# The double-precision helpers to insert before ChildConnectorOriginWorld
DOUBLE_HELPERS = """    private static double DegreesToRadiansD(double deg) => deg * Math.PI / 180.0;

    private static double[] PlaneToMatrixD(Plane p)
    {
        var xAxis = new double[] { p.XAxis.X, p.XAxis.Y, p.XAxis.Z };
        var yAxis = new double[] { p.YAxis.X, p.YAxis.Y, p.YAxis.Z };
        var zAxis = CrossD(xAxis, yAxis);
        NormalizeD(zAxis);
        return new double[]
        {
            xAxis[0], yAxis[0], zAxis[0], p.Origin.X,
            xAxis[1], yAxis[1], zAxis[1], p.Origin.Y,
            xAxis[2], yAxis[2], zAxis[2], p.Origin.Z,
            0, 0, 0, 1,
        };
    }

    private static Plane MatrixToPlaneD(double[] m)
    {
        return new Plane
        {
            Origin = new Point { X = m[3], Y = m[7], Z = m[11] },
            XAxis = new Vector { X = m[0], Y = m[4], Z = m[8] },
            YAxis = new Vector { X = m[1], Y = m[5], Z = m[9] },
        };
    }

    private static double[] ApplyMatrix4ToVec3D(double[] m, double[] v)
    {
        return new double[]
        {
            m[0] * v[0] + m[1] * v[1] + m[2] * v[2],
            m[4] * v[0] + m[5] * v[1] + m[6] * v[2],
            m[8] * v[0] + m[9] * v[1] + m[10] * v[2],
        };
    }

    private static double[] QuaternionFromAxisAngleD(double[] axis, double angle)
    {
        var halfAngle = angle / 2.0;
        var s = Math.Sin(halfAngle);
        return new double[] { axis[0] * s, axis[1] * s, axis[2] * s, Math.Cos(halfAngle) };
    }

    private static double[] QuaternionFromUnitVectorsD(double[] vFrom, double[] vTo)
    {
        var r = DotD(vFrom, vTo) + 1.0;
        double[] quat;
        if (r < 0.000001)
        {
            if (Math.Abs(vFrom[0]) > Math.Abs(vFrom[2]))
                quat = new double[] { -vFrom[1], vFrom[0], 0, 0 };
            else
                quat = new double[] { 0, -vFrom[2], vFrom[1], 0 };
        }
        else
        {
            var crossV = CrossD(vFrom, vTo);
            quat = new double[] { crossV[0], crossV[1], crossV[2], r };
        }
        var length = Math.Sqrt(quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]);
        return new double[] { quat[0] / length, quat[1] / length, quat[2] / length, quat[3] / length };
    }

    private static double[] QuaternionToMatrixD(double[] q)
    {
        double x = q[0], y = q[1], z = q[2], w = q[3];
        double x2 = x + x, y2 = y + y, z2 = z + z;
        double xx = x * x2, xy = x * y2, xz = x * z2;
        double yy = y * y2, yz = y * z2, zz = z * z2;
        double wx = w * x2, wy = w * y2, wz = w * z2;
        return new double[]
        {
            1 - (yy + zz), xy - wz, xz + wy, 0,
            xy + wz, 1 - (xx + zz), yz - wx, 0,
            xz - wy, yz + wx, 1 - (xx + yy), 0,
            0, 0, 0, 1,
        };
    }

    private static double[] MakeTranslationD(double x, double y, double z)
    {
        return new double[]
        {
            1, 0, 0, x,
            0, 1, 0, y,
            0, 0, 1, z,
            0, 0, 0, 1,
        };
    }

    private static double[] MakeRotationAxisD(double[] axis, double angle)
    {
        var c = Math.Cos(angle);
        var s = Math.Sin(angle);
        var t = 1.0 - c;
        double x = axis[0], y = axis[1], z = axis[2];
        return new double[]
        {
            t * x * x + c, t * x * y - s * z, t * x * z + s * y, 0,
            t * x * y + s * z, t * y * y + c, t * y * z - s * x, 0,
            t * x * z - s * y, t * y * z + s * x, t * z * z + c, 0,
            0, 0, 0, 1,
        };
    }

    private static double[] MultiplyMatricesD(double[] a, double[] b)
    {
        var r = new double[16];
        for (int row = 0; row < 4; row++)
            for (int col = 0; col < 4; col++)
            {
                double s = 0;
                for (int k = 0; k < 4; k++)
                    s += a[row * 4 + k] * b[k * 4 + col];
                r[row * 4 + col] = s;
            }
        return r;
    }

    private static Plane ComputeChildPlaneD(Plane parentPlane, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var parentMatrix = PlaneToMatrixD(parentPlane);
        var parentPoint = new double[] { parentConnector.Point?.X ?? 0, parentConnector.Point?.Y ?? 0, parentConnector.Point?.Z ?? 0 };
        var parentDirection = new double[] { parentConnector.Direction?.X ?? 0, parentConnector.Direction?.Y ?? 1, parentConnector.Direction?.Z ?? 0 };
        NormalizeD(parentDirection);
        var childPoint = new double[] { childConnector.Point?.X ?? 0, childConnector.Point?.Y ?? 0, childConnector.Point?.Z ?? 0 };
        var childDirection = new double[] { childConnector.Direction?.X ?? 0, childConnector.Direction?.Y ?? 1, childConnector.Direction?.Z ?? 0 };
        NormalizeD(childDirection);

        var rotationRad = DegreesToRadiansD(connection.Rotation);
        var turnRad = DegreesToRadiansD(connection.Turn);
        var tiltRad = DegreesToRadiansD(connection.Tilt);

        var reverseChildDirection = new double[] { -childDirection[0], -childDirection[1], -childDirection[2] };

        double[] alignQuat;
        var crossVec = CrossD(parentDirection, reverseChildDirection);
        var crossLen = Math.Sqrt(crossVec[0] * crossVec[0] + crossVec[1] * crossVec[1] + crossVec[2] * crossVec[2]);
        if (crossLen < 0.01)
        {
            if (Math.Abs(parentDirection[2]) < 1e-5)
            {
                alignQuat = QuaternionFromAxisAngleD(new double[] { 0, 0, 1 }, Math.PI);
            }
            else
            {
                var axis = CrossD(new double[] { 0, 0, 1 }, parentDirection);
                NormalizeD(axis);
                alignQuat = QuaternionFromAxisAngleD(axis, Math.PI);
            }
        }
        else
        {
            alignQuat = QuaternionFromUnitVectorsD(reverseChildDirection, parentDirection);
        }

        var directionT = QuaternionToMatrixD(alignQuat);

        var yAxis = new double[] { 0, 1, 0 };
        var parentConnectorQuat = QuaternionFromUnitVectorsD(yAxis, parentDirection);
        var parentRotationT = QuaternionToMatrixD(parentConnectorQuat);

        var gapDirection = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 0, 1, 0 });
        var shiftDirection = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 1, 0, 0 });
        var raiseDirection = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 0, 0, 1 });
        var turnAxis = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 0, 0, 1 });
        var tiltAxis = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 1, 0, 0 });

        var orientationT = directionT;

        var rotateT = MakeRotationAxisD(parentDirection, -rotationRad);
        orientationT = MultiplyMatricesD(rotateT, orientationT);

        turnAxis = ApplyMatrix4ToVec3D(rotateT, turnAxis);
        tiltAxis = ApplyMatrix4ToVec3D(rotateT, tiltAxis);

        var turnT = MakeRotationAxisD(turnAxis, turnRad);
        orientationT = MultiplyMatricesD(turnT, orientationT);

        var tiltT = MakeRotationAxisD(tiltAxis, tiltRad);
        orientationT = MultiplyMatricesD(tiltT, orientationT);

        var centerChildT = MakeTranslationD(-childPoint[0], -childPoint[1], -childPoint[2]);
        var transform = MultiplyMatricesD(orientationT, centerChildT);

        var gapT = MakeTranslationD(gapDirection[0] * connection.Gap, gapDirection[1] * connection.Gap, gapDirection[2] * connection.Gap);
        var shiftT = MakeTranslationD(shiftDirection[0] * connection.Shift, shiftDirection[1] * connection.Shift, shiftDirection[2] * connection.Shift);
        var raiseT = MakeTranslationD(raiseDirection[0] * connection.Rise, raiseDirection[1] * connection.Rise, raiseDirection[2] * connection.Rise);

        var translationT = MultiplyMatricesD(raiseT, MultiplyMatricesD(shiftT, gapT));
        transform = MultiplyMatricesD(translationT, transform);
        var moveToParentT = MakeTranslationD(parentPoint[0], parentPoint[1], parentPoint[2]);
        transform = MultiplyMatricesD(moveToParentT, transform);
        var finalMatrix = MultiplyMatricesD(parentMatrix, transform);

        return MatrixToPlaneD(finalMatrix);
    }

"""

# Replace the ChildConnectorOriginWorld to use ComputeChildPlaneD
OLD_CHILD_ORIGIN = """    private static double[] ChildConnectorOriginWorld(Plane parentPlane, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var childPlane = DefaultComputeChildPlane(
            parentPlane,
            parentConnector.Point ?? new Point(),
            parentConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            childConnector.Point ?? new Point(),
            childConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            connection.Gap, connection.Shift, connection.Rise,
            connection.Rotation, connection.Turn, connection.Tilt);
        return new double[] { childPlane.Origin.X, childPlane.Origin.Y, childPlane.Origin.Z };
    }"""

NEW_CHILD_ORIGIN = """    private static double[] ChildConnectorOriginWorld(Plane parentPlane, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var childPlane = ComputeChildPlaneD(parentPlane, parentConnector, childConnector, connection);
        return new double[] { childPlane.Origin.X, childPlane.Origin.Y, childPlane.Origin.Z };
    }"""

# Also update ConnectionPlacementTranslationBasis to use double precision
OLD_CONN_BASIS = """    private static void ConnectionPlacementTranslationBasis(Connector parentConnector, out double[] gapDir, out double[] shiftDir, out double[] raiseDir)
    {
        var parentDirection = new double[] { parentConnector.Direction?.X ?? 0, parentConnector.Direction?.Y ?? 1, parentConnector.Direction?.Z ?? 0 };
        NormalizeD(parentDirection);
        var yAxis = new System.Numerics.Vector3(0, 1, 0);
        var pDir = new System.Numerics.Vector3((float)parentDirection[0], (float)parentDirection[1], (float)parentDirection[2]);
        var parentConnectorQuat = CreateFromTwoVectors(yAxis, pDir);
        var parentRotationT = QuaternionToMatrix(parentConnectorQuat);
        var gapV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitY);
        gapDir = new double[] { gapV.X, gapV.Y, gapV.Z };
        NormalizeD(gapDir);
        var shiftV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitX);
        shiftDir = new double[] { shiftV.X, shiftV.Y, shiftV.Z };
        NormalizeD(shiftDir);
        var raiseV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitZ);
        raiseDir = new double[] { raiseV.X, raiseV.Y, raiseV.Z };
        NormalizeD(raiseDir);
    }"""

NEW_CONN_BASIS = """    private static void ConnectionPlacementTranslationBasis(Connector parentConnector, out double[] gapDir, out double[] shiftDir, out double[] raiseDir)
    {
        var parentDirection = new double[] { parentConnector.Direction?.X ?? 0, parentConnector.Direction?.Y ?? 1, parentConnector.Direction?.Z ?? 0 };
        NormalizeD(parentDirection);
        var yAxisD = new double[] { 0, 1, 0 };
        var parentConnectorQuat = QuaternionFromUnitVectorsD(yAxisD, parentDirection);
        var parentRotationT = QuaternionToMatrixD(parentConnectorQuat);
        gapDir = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 0, 1, 0 });
        NormalizeD(gapDir);
        shiftDir = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 1, 0, 0 });
        NormalizeD(shiftDir);
        raiseDir = ApplyMatrix4ToVec3D(parentRotationT, new double[] { 0, 0, 1 });
        NormalizeD(raiseDir);
    }"""

print(f"Reading {COMPOSE_CS}")
with open(COMPOSE_CS, "r", encoding="utf-8") as f:
    content = f.read()

# Step 1: Insert double helpers before ChildConnectorOriginWorld
idx = content.find(OLD_CHILD_ORIGIN)
if idx == -1:
    print("ERROR: Could not find ChildConnectorOriginWorld")
    exit(1)
print(f"Found ChildConnectorOriginWorld at {idx}")

# Insert helpers right before ChildConnectorOriginWorld
content = (
    content[:idx]
    + DOUBLE_HELPERS
    + NEW_CHILD_ORIGIN
    + content[idx + len(OLD_CHILD_ORIGIN) :]
)
print("Inserted double-precision helpers and updated ChildConnectorOriginWorld")

# Step 2: Replace ConnectionPlacementTranslationBasis
idx2 = content.find(OLD_CONN_BASIS)
if idx2 == -1:
    print("ERROR: Could not find old ConnectionPlacementTranslationBasis")
    exit(1)
content = content[:idx2] + NEW_CONN_BASIS + content[idx2 + len(OLD_CONN_BASIS) :]
print("Updated ConnectionPlacementTranslationBasis to double precision")

# Write
with open(COMPOSE_CS, "w", encoding="utf-8") as f:
    f.write(content)

print("SUCCESS: Added double-precision helpers")

# Verify
with open(COMPOSE_CS, "r", encoding="utf-8") as f:
    verify = f.read()

checks = [
    ("ComputeChildPlaneD", "ComputeChildPlaneD" in verify),
    ("PlaneToMatrixD", "PlaneToMatrixD" in verify),
    ("MultiplyMatricesD", "MultiplyMatricesD" in verify),
    ("QuaternionFromUnitVectorsD", "QuaternionFromUnitVectorsD" in verify),
]
for name, ok in checks:
    print(f"  {name}: {'OK' if ok else 'MISSING'}")
    if not ok:
        exit(1)
print("All verified.")
