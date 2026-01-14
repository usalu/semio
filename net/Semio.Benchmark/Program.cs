using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Numerics;
using Newtonsoft.Json;
using Semio;
using Plane = Semio.Plane;
using Point = Semio.Point;
using Vector = Semio.Vector;

namespace Semio.Benchmark;

class Program
{
    const string AssetsPath = "../../assets/semio";
    const int Iterations = 100;
    const float Tolerance = 1e-5f;

    static T LoadAsset<T>(string filename)
    {
        var path = Path.Combine(AssetsPath, filename);
        if (!System.IO.File.Exists(path)) throw new FileNotFoundException($"Asset not found at {Path.GetFullPath(path)}");
        var json = System.IO.File.ReadAllText(path);
        return JsonConvert.DeserializeObject<T>(json)!;
    }

    static void Bench(string name, Action action)
    {
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < Iterations; i++)
        {
            action();
        }
        sw.Stop();
        double duration = sw.Elapsed.TotalSeconds / Iterations;
        Console.WriteLine($"{name},{duration:F6}");
    }

    static Design FindDesign(Kit kit, string name, string? parentName = null)
    {
        string? parentGuid = null;
        if (parentName != null)
        {
             var p = kit.Designs.FirstOrDefault(d => d.Name == parentName);
             if (p == null) throw new Exception($"Parent {parentName} not found");
             parentGuid = p.Guid;
        }

        var d = kit.Designs.FirstOrDefault(d => d.Name == name && (parentGuid != null ? d.Parent?.Guid == parentGuid : d.Parent == null));
        if (d == null) throw new Exception($"Design {name} not found");
        return d;
    }

    static void Main(string[] args)
    {
        var kitMetabolism = LoadAsset<Kit>("kit_metabolism.json");
        var kitInvalid = LoadAsset<Kit>("kit_invalid.json");

        // 1. Roundtrip/Metabolism
        Bench("Roundtrip/Metabolism", () => {
            // Using JsonConvert for consistent benchmarking if Semio.Extensions not available
            var json = JsonConvert.SerializeObject(kitMetabolism);
            JsonConvert.DeserializeObject<Kit>(json);
        });

        // 2. Flatten Design/Nakagin Capsule Tower
        var d1 = FindDesign(kitMetabolism, "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower", () => {
             d1.Flatten(kitMetabolism.Types, ComputeChildPlane);
        });

        // 3. Flatten Design/Nakagin Capsule Tower/Slanted
        var d2 = FindDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Slanted", () => {
             d2.Flatten(kitMetabolism.Types, ComputeChildPlane);
        });

        // 4. Flatten Design/Nakagin Capsule Tower/Twisted
        var d3 = FindDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Twisted", () => {
             d3.Flatten(kitMetabolism.Types, ComputeChildPlane);
        });

        // 5. Flatten Design/Nakagin Capsule Tower/Dancing
        var d4 = FindDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Dancing", () => {
             d4.Flatten(kitMetabolism.Types, ComputeChildPlane);
        });

         // 6. Flatten Design/Capsule Dream
        var d5 = FindDesign(kitMetabolism, "Capsule Dream");
        Bench("Flatten Design/Capsule Dream", () => {
             d5.Flatten(kitMetabolism.Types, ComputeChildPlane);
        });

        // 7. Validation/Invalid Kit
        Bench("Validation/Invalid Kit", () => {
            SemioValidator.ValidateKit(kitInvalid);
        });

        // 8. Validation/Metabolism
        Bench("Validation/Metabolism", () => {
            SemioValidator.ValidateKit(kitMetabolism);
        });
    }

    // --- Math Implementation ---

    public static Plane ComputeChildPlane(
        Plane parentPlane, 
        Point parentPoint, 
        Vector parentDirection, 
        Point childPoint, 
        Vector childDirection, 
        float gap, 
        float shift, 
        float rise, 
        float rotation, 
        float turn, 
        float tilt)
    {
        var pMatrix = PlaneToMatrix(parentPlane); 
        
        var pPoint = new Vector3(parentPoint.X, parentPoint.Y, parentPoint.Z); 
        var pDir = Vector3.Normalize(new Vector3(parentDirection.X, parentDirection.Y, parentDirection.Z));
        var cPoint = new Vector3(childPoint.X, childPoint.Y, childPoint.Z);
        var cDir = Vector3.Normalize(new Vector3(childDirection.X, childDirection.Y, childDirection.Z));

        var rotationRad = DegreesToRadians(rotation);
        var turnRad = DegreesToRadians(turn);
        var tiltRad = DegreesToRadians(tilt);

        var reverseChildDirection = -cDir;

        Quaternion alignQuat;
        var cross = Vector3.Cross(pDir, reverseChildDirection);
        if (cross.LengthSquared() < 0.0001f) 
        {
            if (Math.Abs(pDir.Z) < Tolerance)
            {
                 alignQuat = Quaternion.CreateFromAxisAngle(Vector3.UnitZ, (float)Math.PI);
            }
            else
            {
                 var axis = Vector3.Normalize(Vector3.Cross(Vector3.UnitZ, pDir));
                 alignQuat = Quaternion.CreateFromAxisAngle(axis, (float)Math.PI);
            }
        }
        else
        {
            alignQuat = CreateFromTwoVectors(reverseChildDirection, pDir);
        }

        var directionT = Matrix4x4.CreateFromQuaternion(alignQuat);
        
        var yAxis = Vector3.UnitY;
        var parentConnectorQuat = CreateFromTwoVectors(yAxis, pDir);
        var parentRotationT = Matrix4x4.CreateFromQuaternion(parentConnectorQuat);

        var gapDirection = Vector3.Transform(Vector3.UnitY, parentRotationT);
        var shiftDirection = Vector3.Transform(Vector3.UnitX, parentRotationT);
        var raiseDirection = Vector3.Transform(Vector3.UnitZ, parentRotationT);
        var turnAxis = Vector3.Transform(Vector3.UnitZ, parentRotationT);
        var tiltAxis = Vector3.Transform(Vector3.UnitX, parentRotationT);

        var orientationT = directionT;
        var rotateT = Matrix4x4.CreateFromAxisAngle(pDir, -rotationRad);
        orientationT = orientationT * rotateT;

        turnAxis = Vector3.Transform(turnAxis, rotateT);
        tiltAxis = Vector3.Transform(tiltAxis, rotateT);

        var turnT = Matrix4x4.CreateFromAxisAngle(turnAxis, turnRad);
        orientationT = orientationT * turnT;

        var tiltT = Matrix4x4.CreateFromAxisAngle(tiltAxis, tiltRad);
        orientationT = orientationT * tiltT;

        var centerChildT = Matrix4x4.CreateTranslation(-cPoint);
        
        var transform = centerChildT * orientationT; 

        var translationVec = (gapDirection * gap) + (shiftDirection * shift) + (raiseDirection * rise);
        var translationT = Matrix4x4.CreateTranslation(translationVec);

        transform = transform * translationT; 

        var moveToParentT = Matrix4x4.CreateTranslation(pPoint);
        transform = transform * moveToParentT; 

        var finalMatrix = transform * pMatrix; 

        return MatrixToPlane(finalMatrix);
    }

    private static float DegreesToRadians(float deg) => deg * (float)Math.PI / 180f;

    private static Quaternion CreateFromTwoVectors(Vector3 u, Vector3 v)
    {
        float dot = Vector3.Dot(u, v);
        if (dot > 0.999999f) return Quaternion.Identity;
        if (dot < -0.999999f)
        {
            var axis = Vector3.Cross(Vector3.UnitX, u);
            if (axis.LengthSquared() < 0.001f)
                axis = Vector3.Cross(Vector3.UnitY, u);
            axis = Vector3.Normalize(axis);
            return Quaternion.CreateFromAxisAngle(axis, (float)Math.PI);
        }

        var axisNorm = Vector3.Cross(u, v);
        var q = new Quaternion(axisNorm.X, axisNorm.Y, axisNorm.Z, 1 + dot);
        return Quaternion.Normalize(q);
    }

    private static Matrix4x4 PlaneToMatrix(Plane p)
    {
        var origin = new Vector3(p.Origin.X, p.Origin.Y, p.Origin.Z);
        var x = Vector3.Normalize(new Vector3(p.XAxis.X, p.XAxis.Y, p.XAxis.Z));
        var yRaw = new Vector3(p.YAxis.X, p.YAxis.Y, p.YAxis.Z);
        
        var z = Vector3.Normalize(Vector3.Cross(x, yRaw));
        var y = Vector3.Normalize(Vector3.Cross(z, x));

        return new Matrix4x4(
            x.X, x.Y, x.Z, 0,
            y.X, y.Y, y.Z, 0,
            z.X, z.Y, z.Z, 0,
            origin.X, origin.Y, origin.Z, 1
        );
    }

    private static Plane MatrixToPlane(Matrix4x4 m)
    {
        var x = new Vector3(m.M11, m.M12, m.M13);
        var y = new Vector3(m.M21, m.M22, m.M23);
        var origin = new Vector3(m.M41, m.M42, m.M43);

        return new Plane
        {
            Origin = new Point { X = origin.X, Y = origin.Y, Z = origin.Z },
            XAxis = new Vector { X = x.X, Y = x.Y, Z = x.Z },
            YAxis = new Vector { X = y.X, Y = y.Y, Z = y.Z }
        };
    }
}
