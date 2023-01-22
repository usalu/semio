// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Utility;

namespace Semio.UI.Grasshopper.Goos
{
    public class RepresentationGoo : SemioGeometricGoo<Representation>
    {
        public RepresentationGoo()
        {
            Value = new Representation();
        }

        public RepresentationGoo(Representation attraction)
        {
            Value = attraction;
        }

        public override IGH_Goo Duplicate() => new RepresentationGoo(Value.Clone());
        public override IGH_GeometricGoo DuplicateGeometry()
        {
            return Converter.Convert(Value);
        }

        public override BoundingBox GetBoundingBox(Transform xform)
        {
            throw new NotImplementedException();
        }

        public override IGH_GeometricGoo Transform(Transform xform)
        {
            throw new NotImplementedException();
        }

        public override IGH_GeometricGoo Morph(SpaceMorph xmorph)
        {
            throw new NotImplementedException();
        }

        public override BoundingBox Boundingbox { get; }
        public override string TypeName => Representation.Descriptor.FullName;
        public override string TypeDescription => Representation.Descriptor.Declaration.LeadingComments;
    }
}