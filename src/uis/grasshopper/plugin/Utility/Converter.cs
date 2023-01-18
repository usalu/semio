using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Plane = Rhino.Geometry.Plane;
using SemioPoint = Semio.Model.V1.Point;
using Quaternion = Rhino.Geometry.Quaternion;
using SemioQuaternion = Semio.Model.V1.Quaternion;
using Grasshopper.Kernel.Geometry;


namespace Semio.UI.Grasshopper.Utility
{
    public static class Converter
    {
        public static Quaternion ToQuaternion(SemioQuaternion quaternion) => new((float)quaternion.X, (float)quaternion.Y, (float)quaternion.Z, (float)quaternion.W);
        public static Vector3 ToVector(SemioPoint point) => new ((float)point.X, (float)point.Y, (float)point.Z);
        public static Vector3 ToVector(Point3d point) => new((float)point.X, (float)point.Y, (float)point.Z);
       
        public static Quaternion Convert(SemioQuaternion quaternion) => new(quaternion.W, quaternion.X, quaternion.Y, quaternion.Z);

        public static SemioQuaternion Convert(Quaternion quaternion) => new()
        {
            W = quaternion.A, X = quaternion.B, Y = quaternion.C, Z = quaternion.D
        };
        public static SemioPoint Convert(Point3d point) => new()
        {
            X = point.X,
            Y = point.Y,
            Z = point.Z
        };

        public static Point3d Convert(SemioPoint point) => new()
        {
            X = point.X,
            Y = point.Y,
            Z = point.Z
        };

        public static Pose Convert(Plane plane)
        {
            Plane rotationPlane = plane.Clone();
            rotationPlane.Origin = Point3d.Origin;
            Transform rotationTransform = Transform.PlaneToPlane(Plane.WorldXY, rotationPlane);
            rotationTransform.GetQuaternion(out var quaternion);
            return new Pose()
            {
                PointOfView = Convert(plane.Origin),View = Convert(quaternion)
            };
        }

        public static Plane Convert(Pose pose)
        {
            Quaternion quaternion = Convert(pose.View);
            Transform transform = quaternion.MatrixForm();
            var xAxis = Vector3d.XAxis;
            xAxis.Transform(transform);
            var yAxis = Vector3d.YAxis;
            yAxis.Transform(transform);
            return new Plane(Convert(pose.PointOfView), xAxis, yAxis);
        }


    }
}
