using FluentAssertions;
using Rhino.Geometry;
using Rhino.Runtime.InProcess;
using Rhino.Test;
using Semio.Geometry.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Utility;
using Point = Semio.Geometry.V1.Point;
using Quaternion = Semio.Geometry.V1.Quaternion;

namespace Semio.UI.Grasshopper.Tests
{
    public class ConverterTest : IDisposable
    {
        RhinoSingletonFixture rhinoSingletonFixture;
        public ConverterTest()
        {
            rhinoSingletonFixture = new RhinoSingletonFixture();
        }

        public void Dispose()
        {
            rhinoSingletonFixture.Dispose();
        }

        [Theory]
        [MemberData(nameof(Convert_Plane_PoseData))]
        public void Convert_Pose_Plane(Pose pose, Plane expectedPlane)
        {
            using (new RhinoCore())
            {
                var convertedPlane = Converter.Convert(pose);
                var originalPose = Converter.Convert(convertedPlane);

                convertedPlane.Should().Be(expectedPlane);
                originalPose.Should().Be(pose);
            }
        }

        // TODO: Find a way to use parametrization without using static member data
        // Reason: RhinoCommon objects are not invokable before RhinoInside is loaded (over a fixture)
        public static IEnumerable<object[]> Convert_Plane_PoseData()
        {
            yield return new object[] {  new Pose(), new Plane() };
            yield return new object[] { new Plane(
                new Point3d() { X = 0, Y = 0, Z = 0 },
                new Vector3d(){X = 1, Y = 0, Z = 0},
                new Vector3d(){X = 0, Y = 1, Z = 0}
                ), new Pose()
                {
                    PointOfView = new Point() { X = 0, Y = 0, Z = 0 },
                    View = new Quaternion() {W = 1, X = 0, Y = 0, Z = 0}
                } };
        }
    }
}