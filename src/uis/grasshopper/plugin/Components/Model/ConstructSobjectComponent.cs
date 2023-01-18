using System;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf.Collections;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using NetMQ.Sockets;
using Rhino.Geometry;
using Semio.Gateway.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructSobjectComponent : GH_Component
    {
        public ConstructSobjectComponent()
          : base("Construct Sobject", "Sobject",
              "Construct a sobject",
              "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            var fields = Sobject.Descriptor.Fields.InFieldNumberOrder();
            FieldDescriptor field;
            field =fields[0];
            pManager.AddTextParameter(field.Name.ToUpper(), field.Name.Length>2 ? field.Name.Remove(2): field.Name, field.Declaration.LeadingComments, GH_ParamAccess.item);
            field = fields[1];
            pManager.AddTextParameter(field.Name.ToUpper(), field.Name.Length > 2 ? field.Name.Remove(2) : field.Name, field.Declaration.LeadingComments, GH_ParamAccess.item);
            field = fields[2];
            pManager.AddParameter(new PoseParam(), field.Name.ToUpper(), field.Name.Length > 2 ? field.Name.Remove(2) : field.Name, field.Declaration.LeadingComments, GH_ParamAccess.item);
            field = fields[3];
            pManager.AddParameter(new ParameterParam(), field.Name.ToUpper(), field.Name.Length > 2 ? field.Name.Remove(2) : field.Name, field.Declaration.LeadingComments,GH_ParamAccess.item);
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            var descriptor = Sobject.Descriptor;
            pManager.AddParameter(new SobjectParam(), descriptor.Name, descriptor.Name.Length > 2 ? descriptor.Name.Remove(2) : descriptor.Name,
                descriptor.Declaration.LeadingComments, GH_ParamAccess.item);
        }


        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string id = "";
            if (!DA.GetData(0, ref id)) return;

            string url = "";
            if (!DA.GetData(1, ref url)) return;

            PoseGoo pose = new();
            if (!DA.GetData(2, ref pose)) return;

            var parameters = new List<ParameterGoo>();
            if (!DA.GetDataList(3, parameters)) return;

            Sobject sobject = new Sobject()
            {
                Id = id,
                Url = url,
                Pose = pose.GetPose()
            };
            sobject.Parameters.Add(new MapField<string, string>
            {
                parameters.ToDictionary(keySelector: m => m.Value.Name, elementSelector: m => m.Value.Value)
            });

            DA.SetData(0, new SobjectGoo(sobject));
        }

        /// <summary>
        /// Provides an Icon for the component.
        /// </summary>
        protected override System.Drawing.Bitmap Icon
        {
            get
            {
                //You can add image files to your project resources and access them like this:
                // return Resources.IconForThisComponent;
                return null;
            }
        }

        /// <summary>
        /// Gets the unique ID for this component. Do not change this ID after release.
        /// </summary>
        public override Guid ComponentGuid
        {
            get { return new Guid("6A1FE23A-C11F-43E3-B609-5A50BE7F2EE3"); }
        }
    }
}