// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Google.Protobuf;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Encoding = System.Text.Encoding;

namespace Semio.UI.Grasshopper.Goos
{
    public class ConnectableGoo : SemioGoo<Connectable>
    {
        public ConnectableGoo()
        {
            Value = new Connectable();
        }

        public ConnectableGoo(Connectable connectable)
        {
            Value = connectable;
        }

        public override IGH_Goo Duplicate() => new ConnectableGoo(Value.Clone());
        public override string TypeName => Connectable.Descriptor.FullName;
        public override string TypeDescription => Connectable.Descriptor.Declaration.LeadingComments;

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Connectable.Parser.ParseJson(text.Value);
                        return true;
                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Connectable.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                            return true;
                        }
                        catch (Exception exception)
                        {
                        }
                    }
                    return false;
                case SobjectGoo sobject:
                    Value = new Connectable()
                    {
                        SobjectId = sobject.Value.Id
                    };
                    return true;
                default:
                    return false;
            }
        }
    }
}