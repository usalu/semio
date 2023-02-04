// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Google.Protobuf;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Utility;
using Encoding = System.Text.Encoding;

namespace Semio.UI.Grasshopper.Goos
{
    public class RepresentationGoo : SemioGeometricGoo<Representation>
    {
        public RepresentationGoo()
        {
            Value = new Representation();
        }

        public RepresentationGoo(Representation representation)
        {
            Value = representation;
        }

        public override IGH_Goo Duplicate() => new RepresentationGoo(Value.Clone());
        public override IGH_GeometricGoo DuplicateGeometry()
        {
            // TODO: TryToCast all Goos into geometric goos
            //return Converter.Convert(Value);
            throw new NotImplementedException();
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

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Representation.Parser.ParseJson(text.Value);
                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Representation.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                        }
                        catch (Exception exception)
                        {
                            return false;
                        }
                    }
                    return true;
                default:
                    return false;
            }

        }
    }
}