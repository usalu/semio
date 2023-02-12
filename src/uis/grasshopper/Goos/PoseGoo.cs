using GH_IO.Serialization;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Utility;

namespace Semio.UI.Grasshopper.Goos
{
    public class PoseGoo : SemioGoo<Pose>
    {
        public PoseGoo()
        {
            Value = new Pose();
        }
        public PoseGoo(Plane plane)
        {
            Value = Converter.Convert(plane);
        }

        public PoseGoo(Pose pose)
        {
            Value = pose;
        }

        public override IGH_Goo Duplicate()=> new PoseGoo(Value.Clone());

        public override string ToString() => Value.ToString();
        public override bool CastTo<Q>(ref Q target)
        {
            if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
            {
                object ptr = new GH_Plane(Converter.Convert(Value));
                target = (Q)ptr;
                return true;
            }
            return false;
        }
        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            Plane plane = new();
            if (GH_Convert.ToPlane(source, ref plane, GH_Conversion.Both))
            {
                Value = Converter.Convert(plane);
                return true;
            }
            return false;
        }
        public override string TypeName => "Pose";
        public override string TypeDescription => "A pose is an orientation.";

    }
}
